#[cfg(test)]
use soma_core::stores::StoreService;

#[tokio::test]
async fn obj_store_s3() {
    // Debug: Print what VS Code's test environment actually sees
    println!("VS Code Test Environment:");
    println!("AWS_ACCESS_KEY_ID: {:?}", std::env::var("AWS_ACCESS_KEY_ID"));
    println!("AWS_SECRET_ACCESS_KEY: {:?}", std::env::var("AWS_SECRET_ACCESS_KEY"));
    println!("AWS_REGION: {:?}", std::env::var("AWS_REGION"));
    println!("S3_BUCKET: {:?}", std::env::var("S3_BUCKET"));
    
    let s3_path = "s3://com.gmail.docarw/test.txt";
    let store = StoreService::from_uri(s3_path).expect("Failed to create S3 store");
    let data = store.get_object(s3_path).await.expect("Failed to get data from S3 store");
    assert_eq!("Hello world".to_string(), String::from_utf8_lossy(&data));
}

#[tokio::test]
async fn obj_store_gcs() {
    let gcs_path = "gs://genre_test_bucket/test.txt";
    let store = StoreService::from_uri(gcs_path).expect("Failed to create GCS store");
    let data = store.get_object(gcs_path).await.expect("Failed to get data from GCS store");
    println!("Data from GCS store: {:?}", data);
    assert_eq!("Hello world".to_string(), String::from_utf8_lossy(&data));
}

#[tokio::test]
async fn obj_store_azure() {
    let azure_path = "az://genreblobs/genre-test-data/test.txt";
    let store = StoreService::from_uri(azure_path).expect("Failed to create Azure store");
    let data = store.get_object(azure_path).await.expect("Failed to get data from Azure store");
    assert_eq!("Hello world".to_string(), String::from_utf8_lossy(&data));
}

#[tokio::test]
async fn obj_store_http() {
    let http_path = "https://s3.us-west-1.amazonaws.com/com.gmail.docarw/test.txt";
    let store = StoreService::from_uri(http_path).expect("Failed to create HTTP store");
    let data = store.get_object(http_path).await.expect("Failed to get data from HTTP store");
    assert_eq!("Hello world".to_string(), String::from_utf8_lossy(&data));
}
