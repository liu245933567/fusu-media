use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct M3u8MergeRequest {
    pub input_path: String,
    pub output_path: String,
    // pub tarns: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub ffmpeg_path: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let ffmpeg_path = std::env::var("FFMPEG_PATH").map_or("".to_string(), |v| v);
        Self { ffmpeg_path }
    }
}
