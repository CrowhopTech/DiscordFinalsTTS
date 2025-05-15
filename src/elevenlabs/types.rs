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

#[derive(poise::ChoiceParameter)]
pub enum SpeechSpeed {
    Slow,
    Normal,
    Fast,
}

#[derive(poise::ChoiceParameter)]
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

    pub fn get_default_voice_settings(&self) -> VoiceSettings {
        VoiceSettings {
            stability: None,
            similarity_boost: None,
            style: Some(self.get_default_style_exaggeration()),
            use_speaker_boost: None,
            speed: Some(self.get_speed(None)),
        }
    }

    pub fn get_default_style_exaggeration(&self) -> f32 {
        match self {
            KnownVoice::Scotty => 0.0,
            KnownVoice::June => 0.0,
        }
    }

    pub fn default_speed(&self) -> SpeechSpeed {
        match self {
            KnownVoice::Scotty => SpeechSpeed::Normal,
            KnownVoice::June => SpeechSpeed::Normal,
        }
    }

    pub fn get_speed(&self, speed: Option<SpeechSpeed>) -> f32 {
        let r_speed = match speed {
            Some(s) => s,
            None => self.default_speed(),
        };
        match self {
            KnownVoice::Scotty => match r_speed {
                SpeechSpeed::Slow => 0.7,
                SpeechSpeed::Normal => 1.0,
                SpeechSpeed::Fast => 1.2,
            },
            KnownVoice::June => match r_speed {
                SpeechSpeed::Slow => 0.7,
                SpeechSpeed::Normal => 1.0,
                SpeechSpeed::Fast => 1.2,
            },
        }
    }
}
