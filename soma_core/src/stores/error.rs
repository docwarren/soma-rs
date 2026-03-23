use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Failed to parse object store URI: {0}")]
    ObjectStoreUriParseError(String),

    #[error("Failed to create object store: {0}")]
    ObjectStoreCreationError(#[from] object_store::Error),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Store not initialized")]
    StoreNotInitialized(String),

    #[error("List error: {0}")]
    ListError(String),

    #[error("Path error: {0}")]
    PathError(#[from] std::io::Error),

    #[error("Put error: {0}")]
    PutError(String)
}
