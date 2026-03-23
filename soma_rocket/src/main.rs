#[macro_use] extern crate rocket;

use soma_core::mongo::model::patient::Patient;
use soma_core::mongo::model::user::User;
use soma_core::sqlite::genes::{self, GeneError};
use soma_core::stores::StoreService;

use rocket::response::content;
use rocket::serde::json::Json;
use rocket::{ Request, catch, catchers, routes };
use rocket::http::{ContentType, Status};
use rocket::serde::json::serde_json;
use rocket_cors::{ CorsOptions, AllowedHeaders, AllowedOrigins };
use rocket_cors::Method;
use serde::Serialize;
use soma_core::utils::{get_search_options, UtilError};

use std::collections::HashSet;
use std::io::Cursor;
use thiserror::Error;

use soma_core::models::FileSearchRequest;
use soma_core::models::gene_coordinate::GeneCoordinate;
use soma_core::services::search::{ SearchError, SearchService };
use crate::mongo::{ Name, PatientService, UserService };
use crate::{search::models::SearchRequest};

pub mod search;
pub mod mongo;

/// File entry with metadata - matches Tauri's FileEntry struct
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub path: String,
    pub last_modified: String,
    pub size: u64,
}

/// JSON error response sent to clients
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal server error")]
    InternalServerError,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Search error: {0}")]
    SearchError(#[from] SearchError),

    #[error("SQLite error: {0}")]
    SqliteError(#[from] rusqlite::Error),

    #[error("Invalid search request: {0}")]
    UtilError(#[from] UtilError),

    #[error("Storage error: {0}")]
    StoreError(String),

    #[error("Gene lookup error: {0}")]
    GeneError(#[from] GeneError),

    #[error("Patient not found: {0}")]
    PatientNotFound(String),

    #[error("User not found: {0}")]
    UserNotFound(String),
}

impl ApiError {
    fn status_code(&self) -> Status {
        match self {
            ApiError::InternalServerError => Status::InternalServerError,
            ApiError::NotFound(_) => Status::NotFound,
            ApiError::BadRequest(_) => Status::BadRequest,
            ApiError::DatabaseError(_) => Status::InternalServerError,
            ApiError::SearchError(_) => Status::BadRequest,
            ApiError::SqliteError(_) => Status::InternalServerError,
            ApiError::UtilError(_) => Status::BadRequest,
            ApiError::StoreError(_) => Status::InternalServerError,
            ApiError::GeneError(_) => Status::NotFound,
            ApiError::PatientNotFound(_) => Status::NotFound,
            ApiError::UserNotFound(_) => Status::NotFound,
        }
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status = self.status_code();
        let error_response = ErrorResponse {
            error: self.to_string(),
            code: status.code,
        };

        let body = serde_json::to_string(&error_response)
            .unwrap_or_else(|_| r#"{"error":"Internal server error","code":500}"#.to_string());

        rocket::Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

#[post("/", data = "<search_request>")]
async fn search_features(search_request: Json<SearchRequest>) -> Result<String, ApiError> {
    let request = search_request.into_inner();
    if !request.path.ends_with(".bam") &&
       !request.path.ends_with(".vcf.gz") &&
       !request.path.ends_with(".bed.gz") &&
       !request.path.ends_with(".bedgraph.gz") &&
       !request.path.ends_with(".gff.gz") &&
       !request.path.ends_with(".fasta") &&
       !request.path.ends_with(".fa") &&
       !request.path.ends_with(".bigwig") &&
       !request.path.ends_with(".bb") &&
       !request.path.ends_with(".bigbed") &&
       !request.path.ends_with(".bw") {
        return Err(ApiError::BadRequest("Unsupported file type".into()));
    }
    if request.coordinates.is_empty() {
        return Err(ApiError::BadRequest("Missing coordinates".into()));
    }
    let search_request = FileSearchRequest::new(request.path, request.coordinates);
    let search_options = get_search_options(search_request)?;
    let result = SearchService::search_features(&search_options).await?;

    Ok(result.lines
    .into_iter()
    .collect::<Vec<String>>()
    .join("\n"))
}

#[post("/", data="<dir_path>")]
async fn list_dir(dir_path: Json<String>) -> Result<Json<Vec<FileEntry>>, ApiError> {
    let dir = dir_path.into_inner();

    let service = StoreService::from_uri(&dir)
        .map_err(|e| ApiError::StoreError(format!("Invalid path {}: {}", dir, e)))?;

    let objects = service.list_objects(&dir)
        .await
        .map_err(|e| ApiError::StoreError(format!("Failed to list {}: {}", dir, e)))?;

    let entries: Vec<FileEntry> = objects
        .into_iter()
        .map(|meta| FileEntry {
            path: meta.location.to_string(),
            last_modified: meta.last_modified.to_rfc3339(),
            size: meta.size,
        })
        .collect();

    Ok(Json(entries))
}

#[get("/<genome>")]
async fn get_gene_symbols(genome: &str) -> Result<Json<Vec<String>>, ApiError> {
    let url = format!("./data/{}-genes.db", genome.to_lowercase());

    let connection = genes::establish_connection(url.clone())
        .map_err(|e| ApiError::DatabaseError(format!("Failed to open {}: {}", url, e)))?;

    let symbols = genes::get_gene_symbols(&connection)?;
    Ok(Json(symbols))
}

#[get("/<genome>/<gene>")]
async fn get_coordinates(gene: &str, genome: &str) -> Result<Json<GeneCoordinate>, ApiError> {
    let url = format!("./data/{}-genes.db", genome.to_lowercase());

    let connection = genes::establish_connection(url.clone())
        .map_err(|e| ApiError::DatabaseError(format!("Failed to open {}: {}", url, e)))?;

    let coord = genes::get_gene_coordinates(&connection, gene)?;
    Ok(Json(coord))
}

#[post("/", data="<patient_id>")]
async fn get_patient_by_id(patient_id: Json<String>) -> Result<Json<Vec<Patient>>, ApiError> {
    let id = patient_id.into_inner();
    let patient_service = PatientService::new();
    patient_service.get_patient_by_id(&id)
        .await
        .map(Json)
        .map_err(|e| ApiError::PatientNotFound(format!("id={}: {}", id, e)))
}

#[post("/", data="<name>")]
async fn get_patient_by_name(name: Json<Name>) -> Result<Json<Vec<Patient>>, ApiError> {
    let patient_service = PatientService::new();
    patient_service.get_patient_by_name(&name.first_name, &name.last_name)
        .await
        .map(Json)
        .map_err(|e| ApiError::PatientNotFound(format!("{} {}: {}", name.first_name, name.last_name, e)))
}

#[post("/", data="<dob>")]
async fn get_patient_by_dob(dob: Json<String>) -> Result<Json<Vec<Patient>>, ApiError> {
    let date = dob.into_inner();
    let patient_service = PatientService::new();
    patient_service.get_patient_by_dob(&date)
        .await
        .map(Json)
        .map_err(|e| ApiError::PatientNotFound(format!("dob={}: {}", date, e)))
}

#[post("/", data="<user_id>")]
async fn get_user_by_id(user_id: Json<String>) -> Result<Json<Vec<User>>, ApiError> {
    let id = user_id.into_inner();
    let user_service = UserService::new();
    user_service.get_user_by_id(&id)
        .await
        .map(Json)
        .map_err(|e| ApiError::UserNotFound(format!("id={}: {}", id, e)))
}

#[post("/", data="<user_email>")]
async fn get_user_by_email(user_email: Json<String>) -> Result<Json<Vec<User>>, ApiError> {
    let email = user_email.into_inner();
    let user_service = UserService::new();
    user_service.get_user_by_email(&email)
        .await
        .map(Json)
        .map_err(|e| ApiError::UserNotFound(format!("email={}: {}", email, e)))
}

#[post("/", data="<name>")]
async fn get_user_by_name(name: Json<Name>) -> Result<Json<Vec<User>>, ApiError> {
    let user_service = UserService::new();
    user_service.get_user_by_name(&name.first_name, &name.last_name)
        .await
        .map(Json)
        .map_err(|e| ApiError::UserNotFound(format!("{} {}: {}", name.first_name, name.last_name, e)))
}

// Register a catcher for unhandled errors
#[catch(404)]
fn not_found_catcher(req: &Request) -> content::RawHtml<String> {
    content::RawHtml(format!("<p>404: Not Found - {}</p>", req.uri()))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let aws_access_key = std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not found in environment");
    let aws_secret_key = std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not found in environment");
    let aws_region = std::env::var("AWS_REGION").expect("AWS_REGION not found in environment");
    let s3_bucket = std::env::var("S3_BUCKET").expect("S3_BUCKET not found in environment");

    let mongo_db_name = std::env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME not found in environment");
    let mongo_username = std::env::var("MONGO_USERNAME").expect("MONGO_USERNAME not found in environment");
    let mongo_password = std::env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD not found in environment");
    let mongo_connection = std::env::var("MONGO_CONNECTION").expect("MONGO_CONNECTION not found in environment");

    let azure_storage_container = std::env::var("AZURE_STORAGE_CONTAINER").expect("AZURE_STORAGE_CONTAINER not found in environment");
    let azure_storage_account = std::env::var("AZURE_STORAGE_ACCOUNT").expect("AZURE_STORAGE_ACCOUNT not found in environment");
    let azure_storage_access_key = std::env::var("AZURE_STORAGE_ACCESS_KEY").expect("AZURE_STORAGE_ACCESS_KEY not found in environment");

    let google_service_account = std::env::var("GOOGLE_SERVICE_ACCOUNT").expect("GOOGLE_SERVICE_ACCOUNT not found in environment");
    let google_bucket = std::env::var("GOOGLE_BUCKET").expect("GOOGLE_BUCKET not found in environment");

    let anthropic_api_key = std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not found in environment");

    unsafe {
        std::env::set_var("AWS_SECRET_ACCESS_KEY", aws_secret_key);
        std::env::set_var("AWS_ACCESS_KEY_ID", aws_access_key);
        std::env::set_var("AWS_REGION", aws_region);
        std::env::set_var("S3_BUCKET", s3_bucket);

        std::env::set_var("MONGO_DB_NAME", mongo_db_name);
        std::env::set_var("MONGO_USERNAME", mongo_username);
        std::env::set_var("MONGO_PASSWORD", mongo_password);
        std::env::set_var("MONGO_CONNECTION", mongo_connection);

        std::env::set_var("AZURE_STORAGE_CONTAINER", azure_storage_container);
        std::env::set_var("AZURE_STORAGE_ACCOUNT", azure_storage_account);
        std::env::set_var("AZURE_STORAGE_ACCESS_KEY", azure_storage_access_key);

        std::env::set_var("GOOGLE_SERVICE_ACCOUNT", google_service_account);
        std::env::set_var("GOOGLE_BUCKET", google_bucket);

        std::env::set_var("ANTHROPIC_API_KEY", anthropic_api_key);
    }


    let allowed_urls: AllowedOrigins = AllowedOrigins::some_regex(&[
        "http://localhost:5173".to_string()
    ]);

    let methods = [
        Method::from(rocket::http::Method::Get),
        Method::from(rocket::http::Method::Post)
    ];
    let allowed_methods: HashSet<Method> = HashSet::from(methods);

    let allowed_headers: AllowedHeaders = AllowedHeaders::some(&["Authorization", "Content-Type"]);

    let cors_options: CorsOptions = CorsOptions::default()
        .allowed_origins(allowed_urls)
        .allowed_methods(allowed_methods)
        .allowed_headers(allowed_headers)
        .allow_credentials(true);

    let cors = cors_options.to_cors().expect("Failed to created cors options");

    rocket::build()
        .attach(cors)
        .mount("/", routes![index])
        .mount("/genes/symbols", routes![get_gene_symbols])
        .mount("/genes/coordinates", routes![get_coordinates])
        .mount("/search", routes![search_features])
        .mount("/patients", routes![get_patient_by_id])
        .mount("/patients/name", routes![get_patient_by_name])
        .mount("/patients/dob", routes![get_patient_by_dob])
        .mount("/users", routes![get_user_by_id])
        .mount("/users/email", routes![get_user_by_email])
        .mount("/users/name", routes![get_user_by_name])
        .mount("/files", routes![list_dir])
        .register("/", catchers![not_found_catcher])
        .launch()
        .await?;
    Ok(())
}
