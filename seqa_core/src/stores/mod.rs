// Copyright 2026 Seqa23
//
// Author: Andrew Warren
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::ops::Range;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use futures::StreamExt;
use log::info;
use object_store::path::Path as ObjectStorePath;
use object_store::{ObjectMeta, ObjectStore, ObjectStoreScheme, PutPayload};

pub mod error;
pub mod store;

use error::StoreError;

/// Cloud-agnostic file access service built on top of the [`object_store`] crate.
///
/// `StoreService` maintains a thread-safe cache of backend clients keyed by
/// scheme and host, so repeated access to the same bucket or host reuses a
/// single [`ObjectStore`] instance.
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
#[derive(Debug, Default)]
pub struct StoreService {
    stores: Mutex<HashMap<String, Arc<dyn ObjectStore>>>,
}

impl StoreService {
    /// Creates an empty `StoreService`. Backends are built lazily on first access.
    pub fn new() -> Self {
        Self::default()
    }

    fn get_store_key_from_bucket(bucket: &str, kind: &ObjectStoreScheme) -> Result<String, StoreError> {
        match kind {
            ObjectStoreScheme::Local => {
                Ok(format!("local-{}", bucket))
            },
            ObjectStoreScheme::AmazonS3 => {
                Ok(format!("s3-{}", bucket))
            },
            ObjectStoreScheme::GoogleCloudStorage => {
                Ok(format!("gc-{}", bucket))
            },
            ObjectStoreScheme::MicrosoftAzure => {
                Ok(format!("az-{}", bucket))
            }
            ObjectStoreScheme::Http => {
                Ok(format!("http-{}", bucket))
            }
            _ => {
                Err(StoreError::ValidationError(
                    "Unsupported store type".into(),
                ))
            }
        }
    }

    fn get_store_cache(&self) -> Result<MutexGuard<HashMap<String, Arc<dyn ObjectStore>>>, StoreError> {
        match self.stores.lock() {
            Ok(cache) => Ok(cache),
            Err(_) => Err(StoreError::ValidationError(
                "Store cache poisoned".into(),
            ))
        }
    }

    fn build_store(&self, bucket: &str, store_kind: &ObjectStoreScheme) -> Result<Arc<dyn ObjectStore>, StoreError> {
        match store_kind {
            ObjectStoreScheme::Local => {
                Ok(Arc::new(store::get_local_store()?))
            },
            ObjectStoreScheme::AmazonS3 => {
                Ok(Arc::new(store::get_s3_store_from_bucket(bucket)?))
            },
            ObjectStoreScheme::GoogleCloudStorage => {
                Ok(Arc::new(store::get_gc_store_from_bucket(bucket)?))
            },
            ObjectStoreScheme::MicrosoftAzure => {
                Ok(Arc::new(store::get_azure_store_from_container(bucket)?))
            }
            ObjectStoreScheme::Http => {
                Ok(Arc::new(store::get_http_store(None)?))
            }
            _ => {
                Err(StoreError::ValidationError(
                    "Unsupported store type".into(),
                ))
            }
        }
    }

    pub fn get_or_create_store(&self, bucket: &str, store_kind: &ObjectStoreScheme) -> Result<Arc<dyn ObjectStore>, StoreError> {
        let store_key = Self::get_store_key_from_bucket(bucket, store_kind)?;
        let mut store_cache = self.get_store_cache()?;
        if let Some(store) = store_cache.get(&store_key) {
            return Ok(Arc::clone(store));
        }
        let store = self.build_store(bucket, store_kind)?;
        store_cache.insert(store_key, Arc::clone(&store));
        Ok(store)
    }

    /// Downloads a byte range from `path` and returns the raw bytes.
    ///
    /// `range` is a half-open byte range `[start, end)`.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError`] on path normalisation failure or storage I/O errors.
    pub async fn get_range(&self, path: &str, range: Range<u64>, bucket: &str, kind: &ObjectStoreScheme) -> Result<Vec<u8>, StoreError> {
        let (_, url) = Self::get_canonical_path(path)?;
        let store = self.get_or_create_store(bucket, kind)?;
        Ok(store.get_range(&url, range).await?.to_vec())
    }

    /// Get file path
    /// Gets a Path object from the string supplied.
    pub fn get_canonical_path(path: &str) -> Result<(ObjectStoreScheme, ObjectStorePath), StoreError> {
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
                    ObjectStoreScheme::MicrosoftAzure => { Ok((scheme, path)) }
                    ObjectStoreScheme::AmazonS3 => { Ok((scheme, path)) }
                    ObjectStoreScheme::GoogleCloudStorage => { Ok((scheme, path)) }
                    ObjectStoreScheme::Http => { Ok((scheme, path)) }
                    ObjectStoreScheme::Local => { Ok((scheme, path)) }
                    _ => {
                        Err(StoreError::ValidationError(
                            "Unsupported store type".into(),
                        ))
                    }
                }
            },
            Err(e) => {
                Err(StoreError::ObjectStoreUriParseError(e.to_string()))
            }
        }
    }

    /// Returns the total size of the object at `path` in bytes.
    pub async fn get_file_size(&self, path: &str, bucket: &str, kind: &ObjectStoreScheme) -> Result<u64, StoreError> {
        let (_, canonical) = Self::get_canonical_path(path)?;
        let store = self.get_or_create_store(bucket, kind)?;
        let meta = store.head(&canonical).await?;
        Ok(meta.size)
    }

    /// Downloads the entire object at `path` and returns its bytes.
    pub async fn get_object(&self, path: &str, bucket: &str, kind: &ObjectStoreScheme) -> Result<Vec<u8>, StoreError> {
        let (_, canonical) = Self::get_canonical_path(path)?;
        let store = self.get_or_create_store(bucket, kind)?;
        let result = store.get(&canonical).await?;
        let bytes = result.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Uploads `contents` to the object at `path`, creating or overwriting it.
    pub async fn put_object(&self, path: &str, contents: &[u8], bucket: &str, kind: &ObjectStoreScheme) -> Result<(), StoreError> {
        let (_, canonical) = Self::get_canonical_path(path)?;
        let store = self.get_or_create_store(bucket, kind)?;

        let payload = PutPayload::from(contents.to_vec());

        store
            .put(&canonical, payload)
            .await
            .map_err(|e| StoreError::PutError(e.to_string()))?;
        info!("success object put");
        Ok(())
    }

    /// Lists all objects whose path begins with `prefix`, returning their metadata.
    pub async fn list_objects(&self, prefix: &str,bucket: &str, kind: &ObjectStoreScheme) -> Result<Vec<ObjectMeta>, StoreError> {
        let (_, canonical) = Self::get_canonical_path(prefix)?;
        let store = self.get_or_create_store(bucket, kind)?;
        let mut results = Vec::new();
        let mut stream = store.list(Some(&canonical));

        while let Some(object) = stream.next().await {
            match object {
                Ok(obj) => {
                    results.push(obj);
                }
                Err(e) => return Err(StoreError::ListError(e.to_string())),
            }
        }

        Ok(results)
    }
}
