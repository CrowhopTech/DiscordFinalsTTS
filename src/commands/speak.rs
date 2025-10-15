use ::poise::CreateReply;
use ::serenity::all::CreateAttachment;
use log::{error, info};
use serenity::all::EditMessage;

use crate::commands::util::get_channel_name;
use crate::elevenlabs::ElevenLabs;
use crate::elevenlabs::media::MP3_44100HZ_128KBPS;
use crate::elevenlabs::types::{
    KnownVoice, SpeechModel, SpeechSpeed, VoiceSettings, get_default_speech_model,
};
use crate::streamutil::write_stream_to_vec_u8;
use crate::types::{Context, Error};

/// Generates some speech using the given voice and posts it as a sound snippet
#[poise::command(slash_command, prefix_command)]
pub async fn speak(
    ctx: Context<'_>,
    #[description = "Voice to use"] voice: KnownVoice,
    #[description = "Text to speak"] text: String,
    #[description = "Speed of the speech"] speed: Option<SpeechSpeed>,
    #[description = "Speech model to use"] model: Option<SpeechModel>,
) -> Result<(), Error> {
    let sent_msg_handle = ctx
        .send(CreateReply::default().content("Generating voice..."))
        .await?;
    let mut sent_msg = sent_msg_handle.into_message().await.map_err(|e| {
        error!(error = e.to_string().as_str(); "Failed to convert message to Message");
        Error::from(e)
    })?;

    let bytes = match generate_speech_bytes(&ctx.data().client, voice, text, speed, model).await {
        Err(e) => {
            ctx.send(CreateReply::default().content(format!("Failed to generate voice: {}", e)))
                .await?;
            return Ok(());
        }
        Ok(b) => b,
    };

    sent_msg
        .edit(
            ctx.http(),
            EditMessage::default()
                .new_attachment(CreateAttachment::bytes(
                    bytes.clone(),
                    "Generated voice.mp3",
                ))
                .content("Generated voice"),
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
    #[description = "Speech model to use"] model: Option<SpeechModel>,
) -> Result<(), Error> {
    let guild = ctx.guild().ok_or("Not in a guild")?.id;
    let sctx = ctx.serenity_context();
    let manager = songbird::get(&sctx)
        .await
        .expect("Songbird Voice client placed in at initialization")
        .clone();

    if let Some(handler_lock) = manager.get(guild) {
        let mut handler = handler_lock.lock().await;

        if let Some(channel) = handler.current_channel() {
            let sent_msg_handle = ctx
                .send(
                    CreateReply::default().content(
                        format!(
                            "Generating voice to speak in channel \"{}\"...",
                            get_channel_name(&ctx, channel)?
                        )
                        .as_str(),
                    ),
                )
                .await?;
            let mut sent_msg = sent_msg_handle.into_message().await.map_err(|e| {
                error!(error = e.to_string().as_str(); "Failed to convert message to Message");
                Error::from(e)
            })?;

            let bytes = match generate_speech_bytes(&ctx.data().client, voice, text, speed, model)
                .await
            {
                Err(e) => {
                    ctx.send(
                        CreateReply::default().content(format!("Failed to generate voice: {}", e)),
                    )
                    .await?;
                    return Ok(());
                }
                Ok(b) => b,
            };
            sent_msg
                .edit(
                    ctx.http(),
                    EditMessage::default()
                        .new_attachment(CreateAttachment::bytes(
                            bytes.clone(),
                            "Generated voice.mp3",
                        ))
                        .content(
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
    model: Option<SpeechModel>,
) -> Result<Vec<u8>, Error> {
    info!(
        voice = voice, speed = speed, text = text.as_str();
        "Generating text"
    );

    Ok(write_stream_to_vec_u8(
        client
            .generate_voice(
                voice.get_id(),
                text.clone(),
                Some(VoiceSettings {
                    speed: Some(voice.get_speed(speed)),
                    ..voice.get_default_voice_settings()
                }),
                model.or_else(get_default_speech_model),
                Some(MP3_44100HZ_128KBPS),
            )
            .await?,
    )
    .await
    .inspect_err(|e| {
        error!(
            voice = voice, speed = speed, text = text.as_str(), error = e.to_string().as_str();
            "Failed to generate text",
        );
    })?)
}
