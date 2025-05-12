use crate::elevenlabs::types;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct VoiceList {
    pub voices: Vec<types::Voice>,
}
