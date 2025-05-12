use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct Voice {
    pub voice_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct VoiceSettings {
    pub stability: Option<f32>,
    pub similarity_boost: Option<f32>,
    pub style: Option<f32>,
    pub use_speaker_boost: Option<bool>,
    pub speed: Option<f32>,
}

pub enum KnownVoice {
    Scotty,
    June,
}

impl KnownVoice {
    pub fn get_id(&self) -> String {
        match self {
            KnownVoice::Scotty => "OzxGhSRE3FmszopZTbZE".to_string(),
            KnownVoice::June => "79931Esd1pNmtJORtUBI".to_string(),
        }
    }
}

pub enum MediaFormat {
    MP3,
}

impl MediaFormat {
    pub fn to_str(&self) -> &str {
        match self {
            MediaFormat::MP3 => "mp3",
        }
    }
}

pub struct OutputFormat(MediaFormat, i32, i32); // Use &str as it's a constant string

pub static DEFAULT_OUTPUT_FORMAT: &OutputFormat = MP3_44100HZ_128KBPS;
pub static MP3_44100HZ_128KBPS: &OutputFormat = &OutputFormat(MediaFormat::MP3, 44100, 128000);

impl OutputFormat {
    #[allow(dead_code)]
    pub fn get_format(&self) -> &MediaFormat {
        &self.0
    }

    #[allow(dead_code)]
    pub fn get_sample_rate(&self) -> i32 {
        self.1
    }

    #[allow(dead_code)]
    pub fn get_bitrate(&self) -> i32 {
        self.2
    }

    pub fn to_string(&self) -> String {
        format!("{}_{}_{}", self.0.to_str(), self.1, self.2)
    }
}
