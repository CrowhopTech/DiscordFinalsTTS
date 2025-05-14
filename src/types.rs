use crate::elevenlabs::ElevenLabs;
use crate::elevenlabs::types::KnownVoice;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub const DISCORD_TOKEN_ENV: &str = "DISCORD_TOKEN";
pub const ELEVENLABS_TOKEN_ENV: &str = "ELEVENLABS_TOKEN";

pub struct HttpKey;

impl ::serenity::prelude::TypeMapKey for HttpKey {
    type Value = reqwest::Client;
}

#[derive(poise::ChoiceParameter)]
pub enum VoiceOption {
    Scotty,
    June,
}

impl Into<KnownVoice> for VoiceOption {
    fn into(self) -> KnownVoice {
        match self {
            VoiceOption::Scotty => KnownVoice::Scotty,
            VoiceOption::June => KnownVoice::June,
        }
    }
}

pub struct Data {
    pub client: ElevenLabs,
} // User data, which is stored and accessible in all command invocations
