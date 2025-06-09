mod commands;
mod elevenlabs;
mod streamutil;
mod types;

use crate::commands::{
    join_leave::{join_voice, leave_voice},
    speak::{speak, speak_vs},
    usage::show_usage,
};
use crate::types::{Data, Error, HttpKey};

use ::log::{error, info};
use ::poise::serenity_prelude as serenity;

// This trait adds the `register_songbird` and `register_songbird_with` methods
// to the client builder below, making it easy to install this voice client.
// The voice client can be retrieved in any command using `songbird::get(ctx).await`.
use ::songbird::SerenityInit;

fn parse_env() -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
    let discord_token: String;
    let elevenlabs_token: String;

    match std::env::var(crate::types::DISCORD_TOKEN_ENV) {
        Ok(token) => {
            discord_token = token;
        }
        Err(_) => {
            return Err(format!(
                "Please set the {} environment variable",
                crate::types::DISCORD_TOKEN_ENV
            )
            .into());
        }
    }

    match std::env::var(crate::types::ELEVENLABS_TOKEN_ENV) {
        Ok(token) => {
            elevenlabs_token = token;
        }
        Err(_) => {
            return Err(format!(
                "Please set the {} environment variable",
                crate::types::ELEVENLABS_TOKEN_ENV
            )
            .into());
        }
    }

    Ok((discord_token, elevenlabs_token))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let (discord_token, elevenlabs_token) = parse_env().map_err(|e| {
        error!(error = e.to_string().as_str(); "Error parsing environment variables");
        e
    })?;

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                speak(),
                speak_vs(),
                join_voice(),
                leave_voice(),
                show_usage(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let el_client = elevenlabs::ElevenLabs::new_from_key(elevenlabs_token.to_string());
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data { client: el_client })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .register_songbird()
        .type_map_insert::<HttpKey>(reqwest::Client::new())
        .await?;
    info!("Starting Finals TTS bot...");
    client.start().await?;

    Ok(())
}
