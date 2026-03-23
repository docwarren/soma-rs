use crate::mongo::constants::USER_COLLECTION_NAME;
use crate::mongo::get_default_db;
/// MOdule for User Collection CRUD operations
use crate::mongo::model::user::User;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{Client, bson};

/// Add a user to the MongoDB database
/// # Arguments
/// * `user` - The user to add
/// # Returns
/// * `Result<(String), mongodb::error::Error>` - Ok with the user ID, Err if there was an error
pub async fn add_user(client: &Client, user: &User) -> Result<String, mongodb::error::Error> {
    let db = get_default_db().map_err(|e| {
        mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get default DB: {}", e),
        ))
    })?;

    let collection = client
        .database(db.as_str())
        .collection::<User>(USER_COLLECTION_NAME);

    let insert_result = collection.insert_one(user).await?;
    match insert_result.inserted_id.as_object_id() {
        Some(object_id) => Ok(object_id.to_hex()),
        None => Err(mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get inserted ObjectId",
        ))),
    }
}

/// remove a user from the MongoDB database
/// # Arguments
/// * `client` - The MongoDB client
/// * `user` - The user to remove
/// # Returns
/// * `Result<(), mongodb::error::Error>` - Ok if the user was removed successfully, Err if there was an error
/// # Errors
/// * If the user ID cannot be parsed as an ObjectId
/// * If the user cannot be removed from the database
pub async fn remove_user(client: &Client, user_id: &String) -> Result<(), mongodb::error::Error> {
    let db = get_default_db().map_err(|e| {
        mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get default DB: {}", e),
        ))
    })?;

    let collection = client
        .database(db.as_str())
        .collection::<User>(USER_COLLECTION_NAME);

    let oid = match bson::oid::ObjectId::parse_str(user_id.clone()) {
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

/// get a user from the MongoDB database
/// # Arguments
/// * `client` - The MongoDB client
/// * `user` - The user to get
/// # Returns
/// * `Result<Vec<User>, mongodb::error::Error>` - Ok with the user, Err if there was an error
/// # Errors
/// * If the user cannot be retrieved from the database
pub async fn get_user(
    client: &Client,
    user: mongodb::bson::Document,
) -> Result<Vec<User>, mongodb::error::Error> {
    let db = get_default_db().map_err(|e| {
        mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get default DB: {}", e),
        ))
    })?;

    let collection = client
        .database(db.as_str())
        .collection::<User>(USER_COLLECTION_NAME);

    let cursor = collection.find(user).await?;
    let result: Vec<User> = cursor.try_collect().await?;
    Ok(result)
}

/// update a user in the MongoDB database
/// # Arguments
/// * `client` - The MongoDB client
/// * `user_id` - The user ID
/// * `user` - The user to update
/// # Returns
/// * `Result<(), mongodb::error::Error>` - Ok if the user was updated successfully, Err if there was an error
/// # Errors
/// * If the user ID cannot be parsed as an ObjectId
/// * If the user cannot be updated in the database
pub async fn update_user(
    client: &Client,
    user_id: &String,
    user: &User,
) -> Result<(), mongodb::error::Error> {
    let db = get_default_db().map_err(|e| {
        mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get default DB: {}", e),
        ))
    })?;

    let collection = client
        .database(db.as_str())
        .collection::<User>(USER_COLLECTION_NAME);

    let oid = bson::oid::ObjectId::parse_str(user_id.clone()).map_err(|e| {
        mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to parse ObjectId: {}", e),
        ))
    })?;

    let filter = doc! { "_id": oid };
    collection.replace_one(filter, user.clone()).await?;
    Ok(())
}

pub async fn get_user_by_id(
    client: &Client,
    user_id: &str,
) -> Result<Vec<User>, mongodb::error::Error> {
    let filter = doc! {
        "_id": bson::oid::ObjectId::parse_str(user_id).map_err(|e| mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to parse ObjectId: {}", e),
        )))?,
    };
    return get_user(client, filter).await;
}

pub async fn get_user_by_name(
    client: &Client,
    first_name: &str,
    last_name: &str,
) -> Result<Vec<User>, mongodb::error::Error> {
    let filter = doc! {
        "first_name": first_name,
        "last_name": last_name,
    };
    return get_user(client, filter).await;
}

pub async fn get_user_by_email(
    client: &Client,
    email: &str,
) -> Result<Vec<User>, mongodb::error::Error> {
    let filter = doc! {
        "email": email,
    };
    return get_user(client, filter).await;
}
