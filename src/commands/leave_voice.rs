use crate::types::{Context, Error};

use ::poise::CreateReply;
use ::serenity::all::ChannelId;

/// Makes the TTS bot leave the current voice channel it's in
#[poise::command(slash_command, prefix_command)]
pub async fn leave_voice(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().ok_or("Not in a guild")?.id;
    let sctx = ctx.serenity_context();

    let manager = songbird::get(&sctx)
        .await
        .expect("Songbird Voice client placed in at initialization")
        .clone();

    if let Some(handler_lock) = manager.get(guild) {
        let mut handler = handler_lock.lock().await;

        match handler.current_channel() {
            Some(channel) => {
                let cid = ChannelId::new(channel.0.get());

                if ctx.guild().expect("a guild").channels.contains_key(&cid) {
                    ctx.say(format!(
                        "Leaving channel {}",
                        &ctx.guild()
                            .expect("a guild")
                            .channels
                            .get(&cid)
                            .expect("a valid channel")
                            .name
                    ))
                    .await?;
                } else {
                    ctx.say(format!("Leaving channel {}", channel.0)).await?;
                }

                handler.leave().await?;
            }
            None => {
                ctx.send(
                    CreateReply::default()
                        .content("Not in a voice channel (no current channel for handler)"),
                )
                .await?;
            }
        }
    } else {
        ctx.send(CreateReply::default().content("Not in a voice channel (failed to get manager)"))
            .await?;
    }

    Ok(())
}
