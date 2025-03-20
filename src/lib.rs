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
}
