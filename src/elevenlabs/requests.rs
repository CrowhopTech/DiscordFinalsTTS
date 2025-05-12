use crate::elevenlabs::types;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct CreateSpeechRequest {
    pub text: String,
    pub model_id: Option<String>,
    pub voice_settings: Option<types::VoiceSettings>,
}
