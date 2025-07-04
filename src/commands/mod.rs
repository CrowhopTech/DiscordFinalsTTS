pub mod join_leave;
pub mod speak;
pub mod usage;

mod util;

use ::log::warn;
use ::serenity::async_trait;
use ::songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};

struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                warn!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}
