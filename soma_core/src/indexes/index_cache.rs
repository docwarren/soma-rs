use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use crate::stores::{StoreService, error::StoreError};

/// Determines the local cache path for an index file based on the data file path.
///
/// For remote files (s3://, az://, gs://, https://), extracts the filename and
/// uses it as the local cache name in the current working directory.
/// For local files, returns the expected index path next to the data file.
///
/// # Arguments
/// * `data_file_path` - Path to the data file (BAM, VCF, etc.)
/// * `index_extension` - Extension for the index file (e.g., ".bai", ".tbi")
///
/// # Returns
/// * Local path where the index file should be cached or exists
pub fn get_local_index_path(data_file_path: &str, index_extension: &str) -> PathBuf {
    // Check if this is a remote URL
    let is_remote = data_file_path.starts_with("s3://")
        || data_file_path.starts_with("az://")
        || data_file_path.starts_with("gs://")
        || data_file_path.starts_with("http://")
        || data_file_path.starts_with("https://");

    if is_remote {
        // Extract filename from URL
        let filename = data_file_path
            .split('/')
            .last()
            .unwrap_or("index");

        // Create local cache path in current directory
        PathBuf::from(format!("./{}{}", filename, index_extension))
    } else {
        // For local files, index should be next to the data file
        PathBuf::from(format!("{}{}", data_file_path, index_extension))
    }
}

/// Checks if an index file exists locally, and if not, downloads it from the remote location.
///
/// # Arguments
/// * `index_path` - The original index path (could be remote)
/// * `index_extension` - Extension for the index file (e.g., ".bai", ".tbi")
/// * `data_file_path` - Path to the data file (used to determine local cache location)
///
/// # Returns
/// * Result containing the bytes of the index file
pub async fn get_or_download_index(
    index_path: &str,
    index_extension: &str,
    data_file_path: &str,
) -> Result<Vec<u8>, StoreError> {
    let local_path = get_local_index_path(data_file_path, index_extension);

    // Check if index exists locally
    if local_path.exists() {
        // Read from local cache
        match fs::read(&local_path).await {
            Ok(bytes) => {
                eprintln!("Using cached index from: {}", local_path.display());
                return Ok(bytes);
            }
            Err(e) => {
                eprintln!("Warning: Failed to read local index {}: {}. Downloading from remote.",
                    local_path.display(), e);
            }
        }
    }

    // Index doesn't exist locally or failed to read - download from remote
    eprintln!("Downloading index from: {}", index_path);
    let store_service = StoreService::from_uri(index_path)?;
    let bytes = store_service.get_object(index_path).await?;

    // Try to cache the downloaded index locally
    if let Err(e) = cache_index_locally(&local_path, &bytes).await {
        eprintln!("Warning: Failed to cache index locally at {}: {}",
            local_path.display(), e);
        // Continue anyway - we have the bytes
    } else {
        eprintln!("Cached index to: {}", local_path.display());
    }

    Ok(bytes)
}

/// Writes index bytes to a local cache file.
///
/// # Arguments
/// * `local_path` - Path where to cache the index
/// * `bytes` - Index file contents
///
/// # Returns
/// * Result indicating success or failure
async fn cache_index_locally(local_path: &Path, bytes: &[u8]) -> Result<(), std::io::Error> {
    // Create parent directories if they don't exist
    if let Some(parent) = local_path.parent() {
        fs::create_dir_all(parent).await?;
    }

    // Write the file
    let mut file = fs::File::create(local_path).await?;
    file.write_all(bytes).await?;
    file.flush().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_index_path_remote_s3() {
        let path = get_local_index_path("s3://bucket/path/to/file.bam", ".bai");
        assert_eq!(path, PathBuf::from("./file.bam.bai"));
    }

    #[test]
    fn test_get_local_index_path_remote_https() {
        let path = get_local_index_path("https://example.com/data/sample.vcf.gz", ".tbi");
        assert_eq!(path, PathBuf::from("./sample.vcf.gz.tbi"));
    }

    #[test]
    fn test_get_local_index_path_remote_azure() {
        let path = get_local_index_path("az://container/folder/file.bam", ".bai");
        assert_eq!(path, PathBuf::from("./file.bam.bai"));
    }

    #[test]
    fn test_get_local_index_path_remote_gcs() {
        let path = get_local_index_path("gs://bucket/data/file.bam", ".bai");
        assert_eq!(path, PathBuf::from("./file.bam.bai"));
    }

    #[test]
    fn test_get_local_index_path_local() {
        let path = get_local_index_path("/local/path/to/file.bam", ".bai");
        assert_eq!(path, PathBuf::from("/local/path/to/file.bam.bai"));
    }

    #[test]
    fn test_get_local_index_path_local_relative() {
        let path = get_local_index_path("./data/file.vcf.gz", ".tbi");
        assert_eq!(path, PathBuf::from("./data/file.vcf.gz.tbi"));
    }
}
