//! A Rust client library for the Freesound API
//!
//! This library provides a convenient interface to interact with the Freesound API,
//! allowing users to search, download and manage sound samples from Freesound.org.

mod client;
mod error;
mod models;

pub use client::{DEFAULT_BASE_URL, FreesoundClient};
pub use error::{FreesoundError, Result};
pub use models::{SearchQueryBuilder, SearchResponse, SortOption, Sound};
