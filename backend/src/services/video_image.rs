use aws_sdk_s3::Client;

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
        format!("{}/main.jpg", uuid)
    }

    /// Retorna el prefijo donde están las escenas
    pub fn scenes_prefix(&self, uuid: &str) -> String {
        format!("{}/images/", uuid)
    }

    /// Lista todas las imágenes de escenas
    pub async fn list_scene_images(
        &self,
        uuid: &str,
    ) -> Result<Vec<String>, aws_sdk_s3::Error> {
        let prefix = self.scenes_prefix(uuid);

        storage::list_keys(&self.client, &self.bucket, Some(&prefix)).await
    }

    /// Obtiene la imagen principal
    pub async fn get_main_image(
        &self,
        uuid: &str,
    ) -> Result<aws_sdk_s3::operation::get_object::GetObjectOutput, aws_sdk_s3::Error> {
        let key = self.main_image_key(uuid);

        storage::download(&self.client, &self.bucket, &key).await
    }

    /// URL temporal para la imagen principal
    pub async fn get_main_image_presigned(
        &self,
        uuid: &str,
        expires_secs: u64,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let key = self.main_image_key(uuid);

        storage::presigned_url(&self.client, &self.bucket, &key, expires_secs).await
    }

    /// URLs temporales para todas las imágenes de escenas
    pub async fn get_scene_images_presigned(
        &self,
        uuid: &str,
        expires_secs: u64,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let keys = self.list_scene_images(uuid).await?;

        let mut urls = Vec::with_capacity(keys.len());

        for key in keys {
            let url =
                storage::presigned_url(&self.client, &self.bucket, &key, expires_secs).await?;

            urls.push(url);
        }

        Ok(urls)
    }
}