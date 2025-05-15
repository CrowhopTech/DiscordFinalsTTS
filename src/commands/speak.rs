use ::poise::CreateReply;
use ::serenity::all::CreateAttachment;

use crate::commands::util::get_channel_name;
use crate::elevenlabs::ElevenLabs;
use crate::elevenlabs::media::MP3_44100HZ_128KBPS;
use crate::elevenlabs::types::{KnownVoice, SpeechSpeed, VoiceSettings};
use crate::streamutil::write_stream_to_vec_u8;
use crate::types::{Context, Error};

/// Generates some speech using the given voice and posts it as a sound snippet
#[poise::command(slash_command, prefix_command)]
pub async fn speak(
    ctx: Context<'_>,
    #[description = "Voice to use"] voice: KnownVoice,
    #[description = "Text to speak"] text: String,
    #[description = "Speed of the speech"] speed: Option<SpeechSpeed>,
) -> Result<(), Error> {
    let bytes = match generate_speech_bytes(&ctx.data().client, voice, text, speed).await {
        Err(e) => {
            ctx.send(CreateReply::default().content(format!("Failed to generate voice: {}", e)))
                .await?;
            return Ok(());
        }
        Ok(b) => b,
    };

    ctx.send(
        CreateReply::default().attachment(CreateAttachment::bytes(bytes, "Generated voice.mp3")),
    )
    .await?;

    Ok(())
}

/// Generates some speech using the given voice and posts it in the currently joined voice channel
#[poise::command(slash_command, prefix_command)]
pub async fn speak_vs(
    ctx: Context<'_>,
    #[description = "Voice to use"] voice: KnownVoice,
    #[description = "Text to speak"] text: String,
    #[description = "Speed of the speech"] speed: Option<SpeechSpeed>,
) -> Result<(), Error> {
    let bytes = match generate_speech_bytes(&ctx.data().client, voice, text, speed).await {
        Err(e) => {
            ctx.send(CreateReply::default().content(format!("Failed to generate voice: {}", e)))
                .await?;
            return Ok(());
        }
        Ok(b) => b,
    };

    let guild = ctx.guild().ok_or("Not in a guild")?.id;
    let sctx = ctx.serenity_context();

    let manager = songbird::get(&sctx)
        .await
        .expect("Songbird Voice client placed in at initialization")
        .clone();

    if let Some(handler_lock) = manager.get(guild) {
        let mut handler = handler_lock.lock().await;

        if let Some(channel) = handler.current_channel() {
            ctx.send(
                CreateReply::default().content(
                    format!(
                        "Speaking in channel \"{}\"",
                        get_channel_name(&ctx, channel)?
                    )
                    .as_str(),
                ),
            )
            .await?;
            let _ = handler.play_input(bytes.into());
        } else {
            ctx.send(CreateReply::default().content("Not in a voice channel"))
                .await?;
            return Ok(());
        }
    } else {
        ctx.send(CreateReply::default().content("Not in a voice channel (failed to get manager)"))
            .await?;
    }

    Ok(())
}

async fn generate_speech_bytes(
    client: &ElevenLabs,
    voice: KnownVoice,
    text: String,
    speed: Option<SpeechSpeed>,
) -> Result<Vec<u8>, Error> {
    let v: KnownVoice = voice.into();

    Ok(write_stream_to_vec_u8(
        client
            .generate_voice(
                v.get_id(),
                text,
                Some(VoiceSettings {
                    speed: Some(v.get_speed(speed)),
                    ..v.get_default_voice_settings()
                }),
                Some(MP3_44100HZ_128KBPS),
            )
            .await?,
    )
    .await?)
}
