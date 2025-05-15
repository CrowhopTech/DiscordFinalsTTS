use crate::elevenlabs::ElevenLabs;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub const DISCORD_TOKEN_ENV: &str = "DISCORD_TOKEN";
pub const ELEVENLABS_TOKEN_ENV: &str = "ELEVENLABS_TOKEN";

pub struct HttpKey;

impl ::serenity::prelude::TypeMapKey for HttpKey {
    type Value = reqwest::Client;
}

pub struct Data {
    pub client: ElevenLabs,
} // User data, which is stored and accessible in all command invocations
