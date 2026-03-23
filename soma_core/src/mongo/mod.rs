pub mod constants;
pub mod model;
pub mod patient_collection;
pub mod user_collection;
use mongodb::Client;
use mongodb::options::{AuthMechanism, ClientOptions, Credential};
use std::env;

/// Connect to the MongoDB database
/// # Returns
/// * `Result<Client, mongodb::error::Error>` - The MongoDB client
/// If there is an error connecting to the database, an error is returned
pub async fn connect() -> mongodb::error::Result<Client> {
    let os_uri = env::var_os("MONGO_CONNECTION").ok_or_else(|| {
        mongodb::error::Error::custom("MONGO_CONNECTION environment variable not set")
    })?;

    let os_username = env::var_os("MONGO_USERNAME").ok_or_else(|| {
        mongodb::error::Error::custom("MONGO_USERNAME environment variable not set")
    })?;

    let os_password = env::var_os("MONGO_PASSWORD").ok_or_else(|| {
        mongodb::error::Error::custom("MONGO_PASSWORD environment variable not set")
    })?;

    let os_db = env::var_os("MONGO_DB_NAME").ok_or_else(|| {
        mongodb::error::Error::custom("MONGO_DB_NAME environment variable not set")
    })?;

    let uri = os_uri.to_str().ok_or_else(|| {
        mongodb::error::Error::custom("Failed to convert MONGO_CONNECTION to string")
    })?;

    let username = os_username.to_str().ok_or_else(|| {
        mongodb::error::Error::custom("Failed to convert MONGO_USERNAME to string")
    })?;

    let password = os_password.to_str().ok_or_else(|| {
        mongodb::error::Error::custom("Failed to convert MONGO_PASSWORD to string")
    })?;

    let db = os_db.to_str().ok_or_else(|| {
        mongodb::error::Error::custom("Failed to convert MONGO_DB_NAME to string")
    })?;

    let mut client_options = ClientOptions::parse(uri).await?;

    let scram_sha_1_cred = Credential::builder()
        .username(username.to_string())
        .password(password.to_string())
        .mechanism(AuthMechanism::ScramSha1)
        .source(db.to_string())
        .build();

    client_options.credential = Some(scram_sha_1_cred);

    let client = Client::with_options(client_options)?;

    Ok(client)
}

/// Get the MongoDB database name
/// # Returns
/// * `String` - The MongoDB database name
pub fn get_default_db() -> Result<String, String> {
    let os_db = env::var_os("MONGO_DB_NAME")
        .ok_or_else(|| "MONGO_DB_NAME environment variable not set".to_string())?;

    let db = os_db
        .to_str()
        .ok_or_else(|| "Failed to convert MONGO_DB_NAME to string".to_string())?;

    Ok(db.to_owned())
}
