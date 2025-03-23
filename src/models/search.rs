use super::sound::Sound;
use serde::Deserialize;

/// Response of a search query containing a list of sounds
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    /// Total number of results
    pub count: i32,
    /// Link to next page of results (null if none)
    pub next: Option<String>,
    /// List of sound results
    pub results: Vec<Sound>,
    /// Link to previous page of results (null if none)
    pub previous: Option<String>,
}

/// Sort options for search results
#[derive(Debug, Clone, Copy)]
pub enum SortOption {
    /// Sort by relevance score (default)
    Score,
    /// Sort by duration, longest first
    DurationDesc,
    /// Sort by duration, shortest first
    DurationAsc,
    /// Sort by creation date, newest first
    CreatedDesc,
    /// Sort by creation date, oldest first
    CreatedAsc,
    /// Sort by downloads, most first
    DownloadsDesc,
    /// Sort by downloads, least first
    DownloadsAsc,
    /// Sort by rating, highest first
    RatingDesc,
    /// Sort by rating, lowest first
    RatingAsc,
}

impl ToString for SortOption {
    fn to_string(&self) -> String {
        match self {
            Self::Score => "score",
            Self::DurationDesc => "duration_desc",
            Self::DurationAsc => "duration_asc",
            Self::CreatedDesc => "created_desc",
            Self::CreatedAsc => "created_asc",
            Self::DownloadsDesc => "downloads_desc",
            Self::DownloadsAsc => "downloads_asc",
            Self::RatingDesc => "rating_desc",
            Self::RatingAsc => "rating_asc",
        }
        .to_string()
    }
}

/// Builder pattern for constructing search queries
#[derive(Debug, Default)]
pub struct SearchQueryBuilder {
    query: Option<String>,
    filter: Option<String>,
    sort: Option<SortOption>,
    group_by_pack: Option<bool>,
    page: Option<i32>,
    page_size: Option<i32>,
    fields: Option<Vec<String>>,
    descriptors: Option<Vec<String>>,
    normalized: Option<bool>,
}

impl SearchQueryBuilder {
    /// Create a new SearchQueryBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the search query text
    pub fn query<S: Into<String>>(mut self, query: S) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Add a filter
    pub fn filter<S: Into<String>>(mut self, filter: S) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Set the sort option
    pub fn sort(mut self, sort: SortOption) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Set whether to group results by pack
    pub fn group_by_pack(mut self, group: bool) -> Self {
        self.group_by_pack = Some(group);
        self
    }

    /// Set the page number
    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the number of results per page
    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }

    /// Set the fields to return
    pub fn fields<I, S>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.fields = Some(fields.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Set the descriptors to return
    pub fn descriptors<I, S>(mut self, descriptors: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.descriptors = Some(descriptors.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Set whether to normalize descriptor values
    pub fn normalized(mut self, normalized: bool) -> Self {
        self.normalized = Some(normalized);
        self
    }

    /// Build the query parameters
    pub fn build(&self) -> Vec<(String, String)> {
        // Changé le type de retour
        let mut params = Vec::new();

        if let Some(ref query) = self.query {
            params.push(("query".to_string(), query.clone()));
        }

        if let Some(ref filter) = self.filter {
            params.push(("filter".to_string(), filter.clone()));
        }

        if let Some(sort) = self.sort {
            params.push(("sort".to_string(), sort.to_string()));
        }

        if let Some(group) = self.group_by_pack {
            params.push((
                "group_by_pack".to_string(),
                if group { "1" } else { "0" }.to_string(),
            ));
        }

        if let Some(page) = self.page {
            params.push(("page".to_string(), page.to_string()));
        }

        if let Some(size) = self.page_size {
            params.push(("page_size".to_string(), size.to_string()));
        }

        if let Some(ref fields) = self.fields {
            params.push(("fields".to_string(), fields.join(",")));
        }

        if let Some(ref descriptors) = self.descriptors {
            params.push(("descriptors".to_string(), descriptors.join(",")));
        }

        if let Some(normalized) = self.normalized {
            params.push((
                "normalized".to_string(),
                if normalized { "1" } else { "0" }.to_string(),
            ));
        }

        params
    }
}
