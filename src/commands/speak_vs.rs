use ::poise::CreateReply;

use crate::elevenlabs::types::{KnownVoice, MP3_44100HZ_128KBPS};
use crate::streamutil::write_stream_to_vec_u8;
use crate::types::{Context, Error, VoiceOption};

/// Generates some speech using the given voice and posts it in the currently joined voice channel
#[poise::command(slash_command, prefix_command)]
pub async fn speak_vs(
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

    let guild = ctx.guild().ok_or("Not in a guild")?.id;
    let sctx = ctx.serenity_context();

    let manager = songbird::get(&sctx)
        .await
        .expect("Songbird Voice client placed in at initialization")
        .clone();

    if let Some(handler_lock) = manager.get(guild) {
        let mut handler = handler_lock.lock().await;

        if let Some(channel) = handler.current_channel() {
            ctx.send(CreateReply::default().content(format!("Speaking in channel {}", channel.0)))
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
