use core::ops::Range;
use futures::StreamExt;
use object_store::path::Path as ObjectStorePath;
use object_store::{ObjectMeta, ObjectStore, ObjectStoreScheme, PutPayload};

pub mod error;
pub mod store;

use error::StoreError;

/// Cloud-agnostic file access service built on top of the [`object_store`] crate.
///
/// `StoreService` wraps a single storage backend and exposes uniform byte-range,
/// full-object, upload, and directory-listing operations.
///
/// # Creating a service
///
/// Use [`StoreService::from_uri`] to auto-detect the backend from the URL scheme:
///
/// ```rust,no_run
/// use seqa_core::stores::StoreService;
///
/// // Local file
/// let svc = StoreService::from_uri("file:///data/sample.bam").unwrap();
///
/// // AWS S3 (requires AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_REGION env vars)
/// let svc = StoreService::from_uri("s3://my-bucket/sample.bam").unwrap();
/// ```
#[derive(Debug)]
pub struct StoreService {
    store: Box<dyn ObjectStore>,
}

impl StoreService {
    /// Creates a `StoreService` for the given [`ObjectStoreScheme`].
    ///
    /// For HTTP stores, use [`StoreService::from_uri`] instead, as the base URL
    /// is required to construct an HTTP client.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError`] when the scheme is unsupported or the backend
    /// cannot be initialised (e.g. missing environment variables for cloud stores).
    pub fn new(service_type: ObjectStoreScheme) -> Result<Self, StoreError> {
        let store = match service_type {
            ObjectStoreScheme::AmazonS3 => {
                store::get_s3_store(None).map(|s3| Box::new(s3) as Box<dyn ObjectStore>)
            }
            ObjectStoreScheme::GoogleCloudStorage => {
                store::get_gc_store(None).map(|gc| Box::new(gc) as Box<dyn ObjectStore>)
            }
            ObjectStoreScheme::MicrosoftAzure => {
                store::get_azure_store(None).map(|az| Box::new(az) as Box<dyn ObjectStore>)
            }
            ObjectStoreScheme::Local => {
                store::get_local_store().map(|local| Box::new(local) as Box<dyn ObjectStore>)
            }
            _ => return Err(StoreError::ValidationError("Unsupported store type. Either we dont support your store or you tried to initialize a HTTP store without the URL".into())),
        };
        Ok(StoreService { store: store? })
    }

    /// Creates a `StoreService` by auto-detecting the storage backend from `path`.
    ///
    /// | Scheme | Backend | Required env vars |
    /// |--------|---------|-------------------|
    /// | `s3://` | AWS S3 | `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_REGION` |
    /// | `az://` | Azure Blob | `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`, `AZURE_STORAGE_ACCOUNT` |
    /// | `gs://` | Google Cloud Storage | `GOOGLE_STORAGE_ACCOUNT`, `GOOGLE_BUCKET` |
    /// | `http://` / `https://` | HTTP | — |
    /// | `file://` | Local filesystem | — |
    ///
    /// # Errors
    ///
    /// Returns [`StoreError`] when the URI cannot be parsed, the scheme is unsupported,
    /// or required environment variables are missing.
    pub fn from_uri(path: &str) -> Result<StoreService, StoreError> {
        let url = path.parse()?;

        let (scheme, _path) = match ObjectStoreScheme::parse(&url) {
            Ok((scheme, path)) => (scheme, path),
            Err(e) => {
                return Err(StoreError::ObjectStoreUriParseError(e.to_string()));
            }
        };

        let service_store = match scheme {
            ObjectStoreScheme::AmazonS3 => {
                store::get_s3_store(Some(path)).map(|s3| Box::new(s3) as Box<dyn ObjectStore>)?
            }
            ObjectStoreScheme::GoogleCloudStorage => {
                store::get_gc_store(None).map(|gc| Box::new(gc) as Box<dyn ObjectStore>)?
            }
            ObjectStoreScheme::MicrosoftAzure => {
                store::get_azure_store(None).map(|az| Box::new(az) as Box<dyn ObjectStore>)?
            }
            ObjectStoreScheme::Http => {
                store::get_http_store(path).map(|http| Box::new(http) as Box<dyn ObjectStore>)?
            }
            ObjectStoreScheme::Local => {
                store::get_local_store().map(|local| Box::new(local) as Box<dyn ObjectStore>)?
            }
            _ => {
                return Err(StoreError::ValidationError("Unsupported store type".into()));
            }
        };

        Ok(StoreService {
            store: service_store,
        })
    }

    /// Returns a reference to the underlying [`ObjectStore`] implementation.
    pub fn get_store(&self) -> &Box<dyn ObjectStore> {
        &self.store
    }

    /// Downloads a byte range from `path` and returns the raw bytes.
    ///
    /// `range` is a half-open byte range `[start, end)`.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError`] on path normalisation failure or storage I/O errors.
    pub async fn get_range(&self, path: &str, range: Range<u64>) -> Result<Vec<u8>, StoreError> {
        let path = match Self::get_canonical_path(path) {
            Ok(path) => path,
            Err(e) => {
                return Err(StoreError::ValidationError(format!(
                    "validation error: {}",
                    e
                )));
            }
        };
        Ok(self
            .get_store()
            .get_range(&object_store::path::Path::from(path), range)
            .await?
            .to_vec())
    }

    /// Get file path
    /// Gets Path object from the string supplied.
    pub fn get_canonical_path(path: &str) -> Result<ObjectStorePath, StoreError> {
        let mut abs_path = path.to_owned();

        if !path.contains("://") {
            // Assume local file path
            let abs_path_buf = std::fs::canonicalize(path)?;
            abs_path = match abs_path_buf.to_str() {
                Some(p) => {
                    format!("file://{}", p)
                },
                None => {
                    return Err(StoreError::ValidationError(
                        "Could not convert path to string".into(),
                    ))
                }
            };
        }

        let url = &abs_path.parse()?;

        match ObjectStoreScheme::parse(url) {
            Ok((scheme, path)) => {
                match scheme {
                    ObjectStoreScheme::MicrosoftAzure => { return Ok(path); }
                    ObjectStoreScheme::AmazonS3 => { return Ok(path); }
                    ObjectStoreScheme::GoogleCloudStorage => { return Ok(path); }
                    ObjectStoreScheme::Http => { return Ok(path); }
                    ObjectStoreScheme::Local => { return Ok(path); }
                    _ => {
                        return Err(StoreError::ValidationError(
                            "Unsupported store type".into(),
                        ))
                    }
                }
            },
            Err(e) => {
                return Err(StoreError::ObjectStoreUriParseError(e.to_string()));
            }
        };
    }

    /// Returns the total size of the object at `path` in bytes.
    pub async fn get_file_size(&self, path: &str) -> Result<u64, StoreError> {
        let path = Self::get_canonical_path(path)?;
        let meta = self.get_store().head(&ObjectStorePath::from(path)).await?;
        Ok(meta.size)
    }

    /// Downloads the entire object at `path` and returns its bytes.
    pub async fn get_object(&self, path: &str) -> Result<Vec<u8>, StoreError> {
        let canonical_path = Self::get_canonical_path(path)?;
        let result = self.get_store().get(&canonical_path).await?;
        let bytes = result.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Uploads `contents` to the object at `path`, creating or overwriting it.
    pub async fn put_object(&self, path: &str, contents: &[u8]) -> Result<(), StoreError> {
        let canonical_path = Self::get_canonical_path(path)?;
        let store = self.get_store();

        // Create a payload from the file content
        let payload = PutPayload::from(contents.to_vec());

        // Upload the object
        store
            .put(&canonical_path, payload)
            .await
            .map_err(|e| StoreError::PutError(e.to_string()))?;
        println!("success object put");
        Ok(())
    }

    /// Lists all objects whose path begins with `prefix`, returning their metadata.
    pub async fn list_objects(&self, prefix: &str) -> Result<Vec<ObjectMeta>, StoreError> {
        let prefix = match Self::get_canonical_path(prefix) {
            Ok(path) => path,
            Err(e) => {
                return Err(StoreError::ValidationError(format!(
                    "validation error: {}",
                    e
                )));
            }
        };
        let mut results = Vec::new();
        let mut stream = self.get_store().list(Some(&ObjectStorePath::from(prefix)));

        while let Some(object) = stream.next().await {
            match object {
                Ok(obj) => {
                    // obj.location is already a Path, convert it to string
                    results.push(obj);
                }
                Err(e) => return Err(StoreError::ListError(e.to_string())),
            }
        }

        Ok(results)
    }
}

