use soma_core::mongo::{model::{patient::Patient, user::User}, patient_collection::{ 
    get_patient_by_dob, get_patient_by_id, get_patient_by_name 
}};
use soma_core::mongo::user_collection::get_user_by_id;

use soma_core::mongo::connect;

pub struct PatientService;

impl PatientService {
    pub fn new() -> Self {
        PatientService
    }

    pub async fn get_patient_by_id(&self, patient_id: &str) -> Result<Vec<Patient>, String> {
        let client= connect().await.map_err(|e| e.to_string())?;
        get_patient_by_id(&client, patient_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_patient_by_name(&self, first_name: &str, last_name: &str) -> Result<Vec<Patient>, String> {
        let client= connect().await.map_err(|e| e.to_string())?;
        get_patient_by_name(&client, first_name, last_name).await.map_err(|e| e.to_string())
    }

    pub async fn get_patient_by_dob(&self, dob: &str) -> Result<Vec<Patient>, String> {
        let client= connect().await.map_err(|e| e.to_string())?;
        get_patient_by_dob(&client, dob).await.map_err(|e| e.to_string())
    }
}

#[derive(serde::Deserialize)]
pub struct Name {
    pub first_name: String,
    pub last_name: String,
}

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        UserService
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Vec<User>, String> {
        let client= connect().await.map_err(|e| e.to_string())?;
        get_user_by_id(&client, user_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Vec<User>, String> {
        let client= connect().await.map_err(|e| e.to_string())?;
        soma_core::mongo::user_collection::get_user_by_email(&client, email).await.map_err(|e| e.to_string())
    }

    pub async fn get_user_by_name(&self, first_name: &str, last_name: &str) -> Result<Vec<User>, String> {
        let client= connect().await.map_err(|e| e.to_string())?;
        soma_core::mongo::user_collection::get_user_by_name(&client, first_name, last_name).await.map_err(|e| e.to_string())
    }
}