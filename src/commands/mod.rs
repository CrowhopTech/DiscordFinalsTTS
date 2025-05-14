pub mod join_voice;
pub mod leave_voice;
pub mod speak;

use poise::serenity_prelude as serenity;
use serenity::async_trait;
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};

struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}
