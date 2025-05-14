use crate::commands::TrackErrorNotifier;
use crate::types::{Context, Error};

use ::poise::{CreateReply, serenity_prelude as serenity};

/// Joins the TTS bot to the given voice channel
#[poise::command(slash_command, prefix_command)]
pub async fn join_voice(
    ctx: Context<'_>,
    #[description = "The channel to join"] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    // Filter to voice channels only: error if it's not a voice channel
    let voice_channel = match channel.kind {
        serenity::ChannelType::Voice => channel,
        _ => {
            ctx.send(CreateReply::default().content("That's not a voice channel!"))
                .await?;
            return Ok(());
        }
    };
    let guild = ctx.guild().ok_or("Not in a guild")?.id;
    let sctx = ctx.serenity_context();

    let manager = songbird::get(&sctx)
        .await
        .expect("Songbird Voice client placed in at initialization")
        .clone();

    if let Ok(handler_lock) = manager.join(guild, voice_channel.id).await {
        // Attach an event handler to see notifications of all track errors.
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(
            ::songbird::events::TrackEvent::Error.into(),
            TrackErrorNotifier,
        );
    } else {
        ctx.send(CreateReply::default().content("Failed to join the voice channel"))
            .await?;
        return Ok(());
    }
    ctx.say(format!("Joined voice channel \"{}\"", voice_channel.name))
        .await?;

    Ok(())
}
