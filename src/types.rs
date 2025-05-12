use crate::elevenlabs::ElevenLabs;
use crate::elevenlabs::types::KnownVoice;

#[derive(poise::ChoiceParameter)]
pub enum VoiceOption {
    Scotty,
    June,
}

impl Into<KnownVoice> for VoiceOption {
    fn into(self) -> KnownVoice {
        match self {
            VoiceOption::Scotty => KnownVoice::Scotty,
            VoiceOption::June => KnownVoice::June,
        }
    }
}

pub struct Data {
    pub client: ElevenLabs,
} // User data, which is stored and accessible in all command invocations
