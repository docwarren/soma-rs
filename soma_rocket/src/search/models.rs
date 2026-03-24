use rocket::serde::{ Deserialize };

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SearchRequest {
    pub coordinates: String,
    pub path: String,
}