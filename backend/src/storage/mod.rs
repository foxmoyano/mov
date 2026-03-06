use std::env;
use std::time::Duration;

use aws_config::Region;
use aws_credential_types::Credentials;
use aws_sdk_s3::{
    config::Builder,
    operation::get_object::GetObjectOutput,
    presigning::PresigningConfig,
    primitives::ByteStream,
    Client,
};

/// Construye un cliente S3 apuntando a MinIO usando variables de entorno:
///   MINIO_ENDPOINT   (ej. http://localhost:9000)
///   MINIO_ACCESS_KEY
///   MINIO_SECRET_KEY
///   MINIO_REGION     (opcional, default "us-east-1")
pub fn build_client() -> Client {
    let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT must be set");
    let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY must be set");
    let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY must be set");
    let region = env::var("MINIO_REGION").unwrap_or_else(|_| "us-east-1".to_string());

    let creds = Credentials::new(access_key, secret_key, None, None, "minio");

    let config = Builder::new()
        .endpoint_url(endpoint)
        .credentials_provider(creds)
        .region(Region::new(region))
        .force_path_style(true) // requerido para MinIO
        .build();

    Client::from_conf(config)
}

/// Sube bytes al bucket indicado y devuelve la key almacenada.
pub async fn upload(
    client: &Client,
    bucket: &str,
    key: &str,
    body: Vec<u8>,
    content_type: &str,
) -> Result<String, aws_sdk_s3::Error> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(body))
        .content_type(content_type)
        .send()
        .await?;

    Ok(key.to_string())
}

/// Descarga un objeto y devuelve el output (incluye body streaming).
pub async fn download(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<GetObjectOutput, aws_sdk_s3::Error> {
    let output = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    Ok(output)
}

/// Genera una URL prefirmada para acceso temporal (lectura).
/// `expires_secs` indica cuántos segundos será válida la URL.
pub async fn presigned_url(
    client: &Client,
    bucket: &str,
    key: &str,
    expires_secs: u64,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let presigning_config = PresigningConfig::expires_in(Duration::from_secs(expires_secs))?;

    let presigned = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .presigned(presigning_config)
        .await?;

    Ok(presigned.uri().to_string())
}

/// Elimina un objeto del bucket.
pub async fn delete(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<(), aws_sdk_s3::Error> {
    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    Ok(())
}

/// Lista las keys de objetos en un bucket con un prefijo opcional.
pub async fn list_keys(
    client: &Client,
    bucket: &str,
    prefix: Option<&str>,
) -> Result<Vec<String>, aws_sdk_s3::Error> {
    let mut req = client.list_objects_v2().bucket(bucket);

    if let Some(p) = prefix {
        req = req.prefix(p);
    }

    let output = req.send().await?;

    let keys = output
        .contents()
        .iter()
        .filter_map(|obj| obj.key().map(|k| k.to_string()))
        .collect();

    Ok(keys)
}
