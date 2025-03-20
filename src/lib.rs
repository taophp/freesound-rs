//! A Rust client library for the Freesound API
//!
//! This library provides a convenient interface to interact with the Freesound API,
//! allowing users to search, download and manage sound samples from Freesound.org.

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const DEFAULT_BASE_URL: &str = "https://freesound.org/apiv2";

#[derive(Error, Debug)]
pub enum FreesoundError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("API error: {0}")]
    ApiError(String),
}

pub type Result<T> = std::result::Result<T, FreesoundError>;

/// Client for interacting with the Freesound API
///
/// # Examples
///
/// ```
/// # use std::env;
/// # use freesound_rs::{FreesoundClient, DEFAULT_BASE_URL};
/// # dotenvy::dotenv().ok();
/// # let api_key = env::var("FREESOUND_API_KEY").expect("FREESOUND_API_KEY must be set");
/// let client = FreesoundClient::new(api_key.clone(), None);
/// assert_eq!(client.base_url(), DEFAULT_BASE_URL);
/// ```
#[derive(Debug, Clone)]
pub struct FreesoundClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl FreesoundClient {
    /// Creates a new Freesound API client
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Freesound API key
    /// * `base_url` - Optional custom base URL for the API. If None, uses the default Freesound API URL.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::env;
    /// # use freesound_rs::{FreesoundClient, DEFAULT_BASE_URL};
    /// # dotenvy::dotenv().ok();
    /// # let api_key = env::var("FREESOUND_API_KEY").expect("FREESOUND_API_KEY must be set");
    /// // With default URL
    /// let client = FreesoundClient::new(api_key.clone(), None);
    /// assert_eq!(client.base_url(), DEFAULT_BASE_URL);
    ///
    /// // With custom URL
    /// let client = FreesoundClient::new(api_key, Some("https://custom.api.url".to_string()));
    /// assert_eq!(client.base_url(), "https://custom.api.url");
    /// ```
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
        }
    }

    /// Returns the API key used by the client
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::env;
    /// # use freesound_rs::FreesoundClient;
    /// # dotenvy::dotenv().ok();
    /// # let api_key = env::var("FREESOUND_API_KEY").expect("FREESOUND_API_KEY must be set");
    /// let client = FreesoundClient::new(api_key.clone(), None);
    /// assert_eq!(client.api_key(), api_key);
    /// ```
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Returns the base URL used by the client
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::env;
    /// # use freesound_rs::{FreesoundClient, DEFAULT_BASE_URL};
    /// # dotenvy::dotenv().ok();
    /// # let api_key = env::var("FREESOUND_API_KEY").expect("FREESOUND_API_KEY must be set");
    /// let client = FreesoundClient::new(api_key, None);
    /// assert_eq!(client.base_url(), DEFAULT_BASE_URL);
    /// ```
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Creates a new authenticated request to the Freesound API
    ///
    /// This method adds the API key as a query parameter to requests
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP method to use
    /// * `path` - API endpoint path (without the base URL)
    ///
    /// # Returns
    ///
    /// A reqwest RequestBuilder with the API key included
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::env;
    /// # use freesound_rs::FreesoundClient;
    /// # use reqwest::Method;
    /// # dotenvy::dotenv().ok();
    /// # let api_key = env::var("FREESOUND_API_KEY").expect("FREESOUND_API_KEY must be set");
    /// # let client = FreesoundClient::new(api_key, None);
    /// let request = client.request(Method::GET, "sounds/1234");
    /// ```
    pub fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        self.client
            .request(method, url)
            .query(&[("token", &self.api_key)])
    }

    /// Performs a test request to verify the API key is valid
    ///
    /// # Returns
    ///
    /// A Result indicating whether the API key is valid
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::env;
    /// # use freesound_rs::{FreesoundClient, FreesoundError};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), FreesoundError> {
    /// dotenvy::dotenv().ok();
    /// let api_key = env::var("FREESOUND_API_KEY").expect("FREESOUND_API_KEY must be set");
    /// let client = FreesoundClient::new(api_key, None);
    ///
    /// // Test avec une clé valide
    /// client.test_api_key().await?;
    /// println!("API key is valid!");
    ///
    /// // Test avec une clé invalide
    /// let invalid_client = FreesoundClient::new("invalid_key".to_string(), None);
    /// let result = invalid_client.test_api_key().await;
    /// assert!(result.is_err());
    /// println!("Invalid API key correctly detected!");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn test_api_key(&self) -> Result<()> {
        let response = self
            .request(reqwest::Method::GET, "sounds/794253")
            .send()
            .await
            .map_err(FreesoundError::from)?;

        let status = response.status();

        // Si le statut est explicitement Unauthorized, on peut directement retourner une erreur
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(FreesoundError::AuthError("Invalid API key".to_string()));
        }

        // Pour d'autres codes d'erreur, on examine le contenu
        if !status.is_success() {
            let body = response.text().await.map_err(FreesoundError::from)?;
            return Err(FreesoundError::ApiError(format!("API request failed: {status} - {body}")));
        }

        // Si on arrive ici, la requête a réussi - on vérifie le JSON
        let body = response.text().await.map_err(FreesoundError::from)?;

        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(json) => {
                if json.get("id").is_some() {
                    Ok(())
                } else {
                    Err(FreesoundError::ApiError(format!("Response missing sound ID: {body}")))
                }
            },
            Err(e) => {
                Err(FreesoundError::ApiError(format!("Invalid JSON response: {e} - {body}")))
            }
        }
    }
}
