use crate::elevenlabs::types;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct VoiceList {
    pub voices: Vec<types::Voice>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct UserInfo {
    pub user_id: String,
    pub subscription: Subscription,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct Subscription {
    pub character_count: i64,
    pub character_limit: i64,
    pub next_character_count_reset_unix: i64,
}
