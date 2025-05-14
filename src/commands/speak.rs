use ::poise::CreateReply;
use ::serenity::all::CreateAttachment;

use crate::elevenlabs::types::{KnownVoice, MP3_44100HZ_128KBPS};
use crate::streamutil::write_stream_to_vec_u8;
use crate::types::{Context, Error, VoiceOption};

/// Generates some speech using the given voice and posts it as a sound snippet
#[poise::command(slash_command, prefix_command)]
pub async fn speak(
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
