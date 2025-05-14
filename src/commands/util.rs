use crate::types::{Context, Error};
use serenity::all::ChannelId as SerenityChannelId;
use songbird::id::ChannelId as SongbirdChannelId;

pub fn get_channel_name(ctx: &Context<'_>, channel: SongbirdChannelId) -> Result<String, Error> {
    let guild = ctx.guild().ok_or("Not in a guild")?;
    Ok(guild
        .channels
        .get(&SerenityChannelId::new(channel.0.get()))
        .ok_or(format!("Channel {} not found", channel.0))?
        .name
        .clone())
}
