use crate::types::{Context, Error};
use chrono::{DateTime, Local};
use ::poise::CreateReply;

#[poise::command(slash_command, prefix_command)]
pub async fn show_usage(ctx: Context<'_>) -> Result<(), Error> {
    let usage = &ctx.data().client.get_usage().await?;
    ctx.send(CreateReply::default().content(format!(
        "**Current ElevenLabs API Usage**\nUsed characters: {} of {}\nResets on: {}",
        usage.0, usage.1,
        DateTime::<Local>::from(usage.2.unwrap_or(DateTime::from_timestamp_nanos(0))).format("%B %d %Y at %r (%Z)")
    )))
    .await?;

    Ok(())
}
