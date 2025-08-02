use crate::api_base::ApiBase;
use anyhow::{anyhow, Result};
use reqwest::{Client, ClientBuilder, Response};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// HTTP client abstraction layer
pub struct ApiClient {
    client: Client,
    config: ApiBase,
}

impl ApiClient {
    pub fn new() -> Self {
        let config = ApiBase::new();
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    pub fn with_config(config: ApiBase) -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Make a GET request
    pub async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.config.endpoint_url(path);
        let response = self.client.get(&url).send().await?;

        self.handle_response(response).await
    }

    /// Make a POST request
    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        let url = self.config.endpoint_url(path);
        let response = self.client.post(&url).json(body).send().await?;

        self.handle_response(response).await
    }

    /// Make a PUT request
    pub async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        let url = self.config.endpoint_url(path);
        let response = self.client.put(&url).json(body).send().await?;

        self.handle_response(response).await
    }

    /// Make a DELETE request
    pub async fn delete<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.config.endpoint_url(path);
        let response = self.client.delete(&url).send().await?;

        self.handle_response(response).await
    }

    /// Handle HTTP response and deserialize JSON
    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!("HTTP {} error: {}", status, error_text));
        }

        let json = response.json::<T>().await?;
        Ok(json)
    }

    /// Get the base configuration
    pub fn config(&self) -> &ApiBase {
        &self.config
    }
}
