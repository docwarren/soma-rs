use crate::mongo::constants::PATIENT_COLLECTION_NAME;
use crate::mongo::model::patient::Patient;
use futures::TryStreamExt;
use mongodb::{
    Client,
    bson::{self, doc},
};

use super::get_default_db;

/// Add a patient to the MongoDB database
/// # Arguments
/// * `patient` - The patient to add
/// # Returns
/// * `Result<(String), mongodb::error::Error>` - Ok with the patient ID, Err if there was an error
pub async fn add_patient(
    client: &Client,
    patient: &Patient,
) -> Result<String, mongodb::error::Error> {
    let db = get_default_db()
        .map_err(|e| mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get default DB: {}", e),
        )))?;

    let collection = client
        .database(db.as_str())
        .collection::<Patient>(PATIENT_COLLECTION_NAME);

    let insert_result = collection.insert_one(patient).await?;
    match insert_result.inserted_id.as_object_id() {
        Some(object_id) => Ok(object_id.to_hex()),
        None => Err(mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get inserted ObjectId",
        ))),
    }
}

/// update a patient already in the db
/// # Arguments
/// - `client` - The MongoDB client
/// - `patient_id` - The patient ID
/// - `patient` - The patient to update
/// # Returns
/// * `Result<(), mongodb::error::Error>` - Ok if the patient was updated successfully, Err if there was an error
/// # Errors
/// * If the patient ID cannot be parsed as an ObjectId
/// * If the patient cannot be updated in the database
pub async fn update_patient(
    client: &Client,
    patient_id: &String,
    patient: &Patient,
) -> Result<(), mongodb::error::Error> {
    let db = get_default_db().map_err(|e| mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get default DB: {}", e),
        )))?;

    let collection = client
        .database(db.as_str())
        .collection::<Patient>(PATIENT_COLLECTION_NAME);

    let oid = bson::oid::ObjectId::parse_str(patient_id.clone()).map_err(|e| mongodb::error::Error::from(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Failed to parse ObjectId: {}", e),
    )))?;

    let filter = doc! { "_id": oid };
    collection.replace_one(filter, patient.clone()).await?;
    Ok(())
}

/// remove a patient from the MongoDB database
/// # Arguments
/// * `client` - The MongoDB client
/// * `patient_id` - The patient_id of the patient to remove
/// # Returns
/// * `Result<(), mongodb::error::Error>` - Ok if the patient was removed successfully, Err if there was an error
/// # Errors
/// * If the patient ID cannot be parsed as an ObjectId
/// * If the patient cannot be removed from the database
pub async fn remove_patient(
    client: &Client,
    patient_id: &String,
) -> Result<(), mongodb::error::Error> {
    let db = get_default_db().map_err(|e| mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get default DB: {}", e),
        )))?;

    let collection = client
        .database(db.as_str())
        .collection::<Patient>(PATIENT_COLLECTION_NAME);

    let oid = match bson::oid::ObjectId::parse_str(patient_id.clone()) {
        Ok(oid) => oid,
        Err(_) => {
            return Err(mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to parse ObjectId",
            )));
        }
    };
    let filter = doc! { "_id": oid };

    collection.delete_one(filter).await?;
    Ok(())
}

/// get a patient from the MongoDB database
/// # Arguments
/// * `client` - The MongoDB client
/// * `patient` - The patient to get (mongo_db::bson::Document) i.e. not the Patient struct
/// # Returns
/// * `Result<Vec<Patient>, mongodb::error::Error>` - Ok with the patient, Err if there was an error
/// # Errors
/// * If the patient ID cannot be parsed as an ObjectId
/// * If the patient cannot be found in the database
pub async fn get_patient(
    client: &Client,
    patient: mongodb::bson::Document,
) -> Result<Vec<Patient>, mongodb::error::Error> {
    let db = get_default_db().map_err(|e| mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get default DB: {}", e),
        )))?;

    let collection = client
        .database(db.as_str())
        .collection::<Patient>(PATIENT_COLLECTION_NAME);

    let cursor = collection.find(patient).await?;
    let result: Vec<Patient> = cursor.try_collect().await?;
    Ok(result)
}

/// Get patient by first and last names
/// # Arguments
/// * connection
/// * first name
/// * last name
/// # Returns
/// * `Result<Vec<Patient>, mongodb::error::Error>` - Ok with the patient, Err if there was an error
pub async fn get_patient_by_name(
    client: &Client,
    first_name: &str,
    last_name: &str,
) -> Result<Vec<Patient>, mongodb::error::Error> {
    let filter = doc! {
        "first_name": first_name,
        "last_name": last_name,
    };
    return get_patient(client, filter).await;
}

pub async fn get_patient_by_dob(
    client: &Client,
    dob: &str,
) -> Result<Vec<Patient>, mongodb::error::Error> {
    let filter = doc! {
        "dob": dob,
    };
    return get_patient(client, filter).await;
}

pub async fn get_patient_by_id(
    client: &Client,
    patient_id: &str,
) -> Result<Vec<Patient>, mongodb::error::Error> {
    let filter = doc! {
        "_id": bson::oid::ObjectId::parse_str(patient_id).map_err(|e| mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to parse ObjectId: {}", e),
        )))?,
    };
    return get_patient(client, filter).await;
}