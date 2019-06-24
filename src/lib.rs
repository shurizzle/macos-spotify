extern crate encoding;
extern crate libc;

mod sys;
#[macro_use]
mod events;
mod spotify;

pub use events::EventBuildError;
pub use spotify::{Spotify, SpotifyTrack, State};
