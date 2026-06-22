use aws_sdk_s3::error::DisplayErrorContext;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use aws_sdk_s3::error::SdkError;
use std::time::Duration;

use crate::storage;

pub struct VideoImageService {
    client: Client,
    bucket: String,
}

impl VideoImageService {
    pub fn new(client: Client, bucket: String) -> Self {
        Self { client, bucket }
    }

    /// Retorna la key del main image
    pub fn main_image_key(&self, uuid: &str) -> String {
        format!("media/{}/main.jpg", uuid)
    }

    /// Retorna el prefijo donde están las escenas
    pub fn scenes_prefix(&self, uuid: &str) -> String {
        format!("media/{}/images/", uuid)
    }

    /// Lista todas las imágenes de escenas
    pub async fn list_scene_images(
        &self,
        uuid: &str,
    ) -> Result<Vec<String>, aws_sdk_s3::Error> {
        let prefix = self.scenes_prefix(uuid);

        tracing::info!(
            bucket = %self.bucket,
            prefix = %prefix,
            "listing scene images in bucket"
        );

        let keys = storage::list_keys(&self.client, &self.bucket, Some(&prefix)).await?;

        tracing::info!(
            bucket = %self.bucket,
            prefix = %prefix,
            count = keys.len(),
            "scene images found in bucket"
        );

        Ok(keys)
    }

    /// Obtiene la imagen principal
    pub async fn get_main_image(
        &self,
        uuid: &str,
    ) -> Result<aws_sdk_s3::operation::get_object::GetObjectOutput, aws_sdk_s3::Error> {
        let key = self.main_image_key(uuid);

        storage::download(&self.client, &self.bucket, &key).await
    }

    /// Verifica si existe la imagen principal usando HEAD
    pub async fn main_image_exists(
        &self,
        uuid: &str,
    ) -> Result<bool, aws_sdk_s3::Error> {
        let key = self.main_image_key(uuid);

        tracing::info!(
            bucket = %self.bucket,
            key = %key,
            "checking main image existence in bucket"
        );

        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await
        {
            Ok(_) => {
                tracing::info!(
                    bucket = %self.bucket,
                    key = %key,
                    "main image exists in bucket"
                );
                Ok(true)
            }
            Err(err) => {
                if is_not_found(&err) {
                    tracing::warn!(
                        bucket = %self.bucket,
                        key = %key,
                        "main image does not exist in bucket"
                    );
                    Ok(false)
                } else {
                    tracing::warn!(
                        bucket = %self.bucket,
                        key = %key,
                        error = %DisplayErrorContext(&err),
                        "error checking main image existence in bucket"
                    );
                    Err(err.into())
                }
            }
        }
    }

    /// URL temporal para la imagen principal, solo si existe realmente
    pub async fn get_main_image_presigned_if_exists(
        &self,
        uuid: &str,
        expires_secs: u64,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        let key = self.main_image_key(uuid);

        let exists = self.main_image_exists(uuid).await?;

        if !exists {
            return Ok(None);
        }

        tracing::info!(
            bucket = %self.bucket,
            key = %key,
            "generating presigned URL for main image"
        );

        let url = storage::presigned_url(&self.client, &self.bucket, &key, expires_secs).await?;

        tracing::info!(
            bucket = %self.bucket,
            key = %key,
            "presigned URL for main image generated"
        );

        Ok(Some(url))
    }

    /// URLs temporales para todas las imágenes de escenas
    pub async fn get_scene_images_presigned(
        &self,
        uuid: &str,
        expires_secs: u64,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let keys = self.list_scene_images(uuid).await?;

        tracing::info!(
            uuid = %uuid,
            count = keys.len(),
            "generating presigned URLs for scene images"
        );

        let mut urls = Vec::with_capacity(keys.len());

        for key in &keys {
            let url = storage::presigned_url(&self.client, &self.bucket, key, expires_secs).await?;
            urls.push(url);
        }

        tracing::info!(
            uuid = %uuid,
            count = urls.len(),
            "presigned URLs for scene images generated"
        );

        Ok(urls)
    }
}

/// Determina si el error de HEAD corresponde a objeto inexistente
fn is_not_found(err: &SdkError<HeadObjectError>) -> bool {
    match err {
        SdkError::ServiceError(service_err) => {
            let code = service_err.err().meta().code().unwrap_or_default();

            code.eq_ignore_ascii_case("NotFound")
                || code.eq_ignore_ascii_case("NoSuchKey")
                || code.eq_ignore_ascii_case("404")
        }
        _ => false,
    }
}