use serde::Deserialize;

/// Preview URLs for different formats and qualities
#[derive(Debug, Deserialize)]
pub struct Previews {
    /// High quality MP3 preview (~128kbps)
    #[serde(rename = "preview-hq-mp3")]
    pub preview_hq_mp3: String,
    /// Low quality MP3 preview (~64kbps)
    #[serde(rename = "preview-lq-mp3")]
    pub preview_lq_mp3: String,
    /// High quality OGG preview (~192kbps)
    #[serde(rename = "preview-hq-ogg")]
    pub preview_hq_ogg: String,
    /// Low quality OGG preview (~80kbps)
    #[serde(rename = "preview-lq-ogg")]
    pub preview_lq_ogg: String,
}

/// Image URLs for sound visualization
#[derive(Debug, Deserialize)]
pub struct Images {
    /// Large waveform image
    pub waveform_l: String,
    /// Medium waveform image
    pub waveform_m: String,
    /// Large spectral image
    pub spectral_l: String,
    /// Medium spectral image
    pub spectral_m: String,
}

/// A sound from Freesound
#[derive(Debug, Deserialize)]
#[serde(default)] // Utilise les valeurs par défaut si les champs sont manquants
pub struct Sound {
    /// The sound's unique identifier
    pub id: i32,
    /// The URI for this sound on the Freesound website
    #[serde(default)]
    pub url: String,
    /// The name user gave to the sound
    pub name: String,
    /// An array of tags the user gave to the sound
    #[serde(default)]
    pub tags: Vec<String>,
    /// The description the user gave to the sound
    #[serde(default)]
    pub description: String,
    /// Latitude and longitude of the geotag (if available)
    pub geotag: Option<String>,
    /// The date when the sound was uploaded
    #[serde(default)]
    pub created: String,
    /// The license under which the sound is available
    pub license: String,
    /// The type of sound (wav, aif, aiff, mp3, m4a or flac)
    #[serde(rename = "type", default)]
    pub sound_type: String,
    /// The number of channels
    #[serde(default)]
    pub channels: i32,
    /// The size of the file in bytes
    #[serde(default)]
    pub filesize: i32,
    /// The bit rate of the sound in kbps
    pub bitrate: Option<f32>,
    /// The bit depth of the sound
    pub bitdepth: Option<i32>,
    /// The duration of the sound in seconds
    #[serde(default)]
    pub duration: f32,
    /// The samplerate of the sound
    #[serde(default)]
    pub samplerate: f32,
    /// The username of the uploader
    pub username: String,
    /// URI pointing to the pack API resource
    pub pack: Option<String>,
    /// URI for downloading the original sound
    #[serde(default)]
    pub download: String,
    /// URI for bookmarking the sound
    #[serde(default)]
    pub bookmark: String,
    /// URLs for mp3 and ogg versions of the sound
    #[serde(default)]
    pub previews: Option<Previews>,
    /// URLs for spectrogram and waveform visualizations
    #[serde(default)]
    pub images: Option<Images>,
    /// The number of times the sound was downloaded
    #[serde(default)]
    pub num_downloads: i32,
    /// The average rating of the sound
    #[serde(default)]
    pub avg_rating: f32,
    /// The number of times the sound was rated
    #[serde(default)]
    pub num_ratings: i32,
    /// The URI for rating the sound
    #[serde(default)]
    pub rate: String,
    /// The URI for the comments
    #[serde(default)]
    pub comments: String,
    /// The number of comments
    #[serde(default)]
    pub num_comments: i32,
    /// The URI to comment the sound
    #[serde(default)]
    pub comment: String,
    /// URI for similar sounds
    #[serde(default)]
    pub similar_sounds: String,
    /// Analysis data (when requested)
    pub analysis: Option<serde_json::Value>,
    /// URI for complete analysis results
    #[serde(default)]
    pub analysis_stats: String,
    /// URI for analysis frames
    #[serde(default)]
    pub analysis_frames: String,
}

impl Default for Sound {
    fn default() -> Self {
        Self {
            id: 0,
            url: String::new(),
            name: String::new(),
            tags: Vec::new(),
            description: String::new(),
            geotag: None,
            created: String::new(),
            license: String::new(),
            sound_type: String::new(),
            channels: 0,
            filesize: 0,
            bitrate: None,
            bitdepth: None,
            duration: 0.0,
            samplerate: 0.0,
            username: String::new(),
            pack: None,
            download: String::new(),
            bookmark: String::new(),
            previews: None,
            images: None,
            num_downloads: 0,
            avg_rating: 0.0,
            num_ratings: 0,
            rate: String::new(),
            comments: String::new(),
            num_comments: 0,
            comment: String::new(),
            similar_sounds: String::new(),
            analysis: None,
            analysis_stats: String::new(),
            analysis_frames: String::new(),
        }
    }
}
