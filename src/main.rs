mod elevenlabs;
mod streamutil;
mod types;

use ::poise::{CreateReply, serenity_prelude as serenity};
use ::serenity::all::CreateAttachment;

use crate::elevenlabs::types::{KnownVoice, MP3_44100HZ_128KBPS};
use crate::streamutil::write_stream_to_vec_u8;
use crate::types::Data;
use crate::types::VoiceOption;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const DISCORD_TOKEN_ENV: &str = "DISCORD_TOKEN";
const ELEVENLABS_TOKEN_ENV: &str = "ELEVENLABS_TOKEN";

/// Generates some speech using the given voice and posts it as a sound snippet
#[poise::command(slash_command, prefix_command)]
async fn speak(
    ctx: Context<'_>,
    #[description = "Voice to use"] voice: VoiceOption,
    #[description = "Text to speak"] text: String,
) -> Result<(), Error> {
    let v: KnownVoice = voice.into();
    let gen_res = ctx
        .data()
        .client
        .generate_voice(v.get_id(), text, Some(MP3_44100HZ_128KBPS))
        .await;
    if let Err(e) = gen_res {
        ctx.send(CreateReply::default().content(format!("Error: {:?}", e)))
            .await?;
        return Ok(());
    }
    let generated = gen_res.unwrap();

    let bytes = write_stream_to_vec_u8(generated).await?;

    ctx.send(
        CreateReply::default().attachment(CreateAttachment::bytes(bytes, "Generated voice.mp3")),
    )
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let discord_token = std::env::var(DISCORD_TOKEN_ENV)
        .unwrap_or_else(|_| panic!("Please set the {} environment variable", DISCORD_TOKEN_ENV));
    let elevenlabs_token = std::env::var(ELEVENLABS_TOKEN_ENV).unwrap_or_else(|_| {
        panic!(
            "Please set the {} environment variable",
            ELEVENLABS_TOKEN_ENV
        )
    });

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![speak()],
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

    println!("Token: {}", discord_token);
    let mut client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await?;
    client.start().await?;

    Ok(())
}
