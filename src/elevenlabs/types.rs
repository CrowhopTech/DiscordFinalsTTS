use log::{error, kv::ToValue};
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

#[derive(poise::ChoiceParameter, Debug, Clone, Copy)]
pub enum SpeechSpeed {
    Slow,
    Normal,
    Fast,
}

impl SpeechSpeed {
    pub fn get_env_name(&self) -> String {
        match self {
            SpeechSpeed::Slow => "SLOW",
            SpeechSpeed::Normal => "NORMAL",
            SpeechSpeed::Fast => "FAST",
        }
        .to_string()
    }
}

impl ToValue for SpeechSpeed {
    fn to_value(&self) -> log::kv::Value {
        match self {
            SpeechSpeed::Slow => "Slow".to_value(),
            SpeechSpeed::Normal => "Normal".to_value(),
            SpeechSpeed::Fast => "Fast".to_value(),
        }
    }
}

#[derive(poise::ChoiceParameter, Debug)]
pub enum KnownVoice {
    Scotty,
    June,
    UnrealTournament,
}

impl ToValue for KnownVoice {
    fn to_value(&self) -> log::kv::Value {
        match self {
            KnownVoice::Scotty => "Scotty".to_value(),
            KnownVoice::June => "June".to_value(),
            KnownVoice::UnrealTournament => "UnrealTournament".to_value(),
        }
    }
}

fn get_env_f32(env_name: &str) -> Option<f32> {
    // Check if the environment variable is set
    if let Ok(env_value) = std::env::var(env_name) {
        // Parse the environment variable as an f32 and return it
        if let Ok(value) = env_value.parse::<f32>() {
            return Some(value);
        }
        error!(
            "Failed to parse env variable {}={} as f32",
            env_name, env_value
        );
    }
    None
}

impl KnownVoice {
    pub fn get_env_name(&self) -> String {
        match self {
            KnownVoice::Scotty => "SCOTTY",
            KnownVoice::June => "JUNE",
            KnownVoice::UnrealTournament => "UNREAL_TOURNAMENT",
        }
        .to_string()
    }

    pub fn get_id(&self) -> String {
        match self {
            KnownVoice::Scotty => "OzxGhSRE3FmszopZTbZE",
            KnownVoice::June => "79931Esd1pNmtJORtUBI",
            KnownVoice::UnrealTournament => "YOq2y2Up4RgXP2HyXjE5",
        }
        .to_string()
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
            KnownVoice::UnrealTournament => 0.0,
        }
    }

    pub fn default_speed(&self) -> SpeechSpeed {
        match self {
            KnownVoice::Scotty => SpeechSpeed::Normal,
            KnownVoice::June => SpeechSpeed::Normal,
            KnownVoice::UnrealTournament => SpeechSpeed::Normal,
        }
    }

    pub fn get_speed_override(&self, speed: SpeechSpeed) -> Option<f32> {
        // First, check if there is an override for this specific voice and speed
        // If not, then check if there is an override for this speed for all voices
        // If not, then return None
        if let Some(override_speed) = get_env_f32(&format!(
            "VOICE_SPEED_OVERRIDE_{}_{}",
            speed.get_env_name(),
            self.get_env_name(),
        )) {
            if override_speed < 0.7 || override_speed > 1.2 {
                error!(
                    "Invalid speed override for {}: {}",
                    self.get_env_name(),
                    override_speed
                );
                return None;
            }
            return Some(override_speed);
        }

        if let Some(override_speed) =
            get_env_f32(&format!("VOICE_SPEED_OVERRIDE_{}_ALL", self.get_env_name(),))
        {
            if override_speed < 0.7 || override_speed > 1.2 {
                error!(
                    "Invalid speed override for {}: {}",
                    self.get_env_name(),
                    override_speed
                );
                return None;
            }
            return Some(override_speed);
        }

        None
    }

    pub fn get_speed(&self, speed: Option<SpeechSpeed>) -> f32 {
        let r_speed = match speed {
            Some(s) => s,
            None => self.default_speed(),
        };
        if let Some(override_speed) = self.get_speed_override(r_speed) {
            return override_speed;
        }

        match self {
            KnownVoice::Scotty => match r_speed {
                SpeechSpeed::Slow => 0.7,
                SpeechSpeed::Normal => 0.85,
                SpeechSpeed::Fast => 1.2,
            },
            KnownVoice::June => match r_speed {
                SpeechSpeed::Slow => 0.7,
                SpeechSpeed::Normal => 0.85,
                SpeechSpeed::Fast => 1.2,
            },
            KnownVoice::UnrealTournament => match r_speed {
                SpeechSpeed::Slow => 0.9,
                SpeechSpeed::Normal => 1.1,
                SpeechSpeed::Fast => 1.2,
            },
        }
    }
}
