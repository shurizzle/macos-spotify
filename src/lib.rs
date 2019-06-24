extern crate encoding;
extern crate libc;

mod sys;
#[macro_use]
mod events;
mod spotify;

pub use spotify::{Spotify, State, SpotifyTrack};
pub use events::EventBuildError;
