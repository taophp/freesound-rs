use crate::error::{FreesoundError, Result};
use crate::models::{SearchResponse, Sound};
use reqwest;

pub const DEFAULT_BASE_URL: &str = "https://freesound.org/apiv2";

/// Client for interacting with the [Freesound API](https://freesound.org/docs/api/)
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
            return Err(FreesoundError::ApiError(format!(
                "API request failed: {status} - {body}"
            )));
        }

        // Si on arrive ici, la requête a réussi - on vérifie le JSON
        let body = response.text().await.map_err(FreesoundError::from)?;

        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(json) => {
                if json.get("id").is_some() {
                    Ok(())
                } else {
                    Err(FreesoundError::ApiError(format!(
                        "Response missing sound ID: {body}"
                    )))
                }
            }
            Err(e) => Err(FreesoundError::ApiError(format!(
                "Invalid JSON response: {e} - {body}"
            ))),
        }
    }
    /// Search for sounds using text query
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    /// use freesound_rs::{FreesoundClient,SearchResponse, SortOption, SearchQueryBuilder};
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// dotenvy::dotenv()?;
    /// let api_key = env::var("FREESOUND_API_KEY").expect("FREESOUND_API_KEY must be set");
    /// let client = FreesoundClient::new(api_key, None);
    /// // Simple search
    /// let query = SearchQueryBuilder::new()
    ///     .query("piano")
    ///     .build();
    /// let results = client.search(&query).await?;
    /// println!("Found {} sounds", results.count);
    ///
    /// // Advanced search
    /// let query = SearchQueryBuilder::new()
    ///     .query("music")
    ///     .filter("tag:guitar")
    ///     .sort(SortOption::RatingDesc)
    ///     .page(1)
    ///     .page_size(15)
    ///     .fields(["id", "name", "tags"])
    ///     .build();
    /// let results = client.search(&query).await?;
    ///  Ok(())
    ///  }
    /// ```
    pub async fn search(&self, query: &[(String, String)]) -> Result<SearchResponse> {
        let response = self
            .request(reqwest::Method::GET, "search/text")
            .query(query)
            .send()
            .await
            .map_err(FreesoundError::from)?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.map_err(FreesoundError::from)?;
            return Err(FreesoundError::ApiError(format!(
                "API request failed: {status} - {body}"
            )));
        }

        response
            .json::<SearchResponse>()
            .await
            .map_err(FreesoundError::from)
    }

    /// Get detailed information about a specific sound
    ///
    /// # Arguments
    ///
    /// * `sound_id` - The unique identifier of the sound
    /// * `descriptors` - Optional list of descriptors to include in the response
    /// * `normalized` - Whether to normalize the descriptor values (only applies if descriptors are requested)
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    /// use freesound_rs::FreesoundClient;
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     dotenvy::dotenv().ok();
    ///     let api_key = env::var("FREESOUND_API_KEY").expect("FREESOUND_API_KEY must be set");
    ///     let client = FreesoundClient::new(api_key, None);
    /// // Get basic sound information
    /// let sound = client.get_sound(1234, None, None).await?;
    /// println!("Sound name: {}", sound.name);
    ///
    /// // Get sound with specific descriptors
    /// let sound = client
    ///     .get_sound(
    ///         1234,
    ///         Some(&["lowlevel.mfcc", "rhythm.bpm"]),
    ///         Some(true)
    ///     )
    ///     .await?;
    ///      Ok(())
    ///  }
    /// ```
    pub async fn get_sound(
        &self,
        sound_id: i32,
        descriptors: Option<&[&str]>,
        normalized: Option<bool>,
    ) -> Result<Sound> {
        let mut request = self.request(reqwest::Method::GET, &format!("sounds/{}", sound_id));

        if let Some(desc) = descriptors {
            request = request.query(&[("descriptors", desc.join(","))]);
        }

        if let Some(norm) = normalized {
            request = request.query(&[("normalized", if norm { "1" } else { "0" })]);
        }

        let response = request.send().await.map_err(FreesoundError::from)?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.map_err(FreesoundError::from)?;
            return Err(FreesoundError::ApiError(format!(
                "API request failed: {status} - {body}"
            )));
        }

        response.json::<Sound>().await.map_err(FreesoundError::from)
    }
}
