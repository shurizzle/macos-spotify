extern crate four_char_code;
extern crate encoding;
extern crate libc;

#[macro_use]
mod sys;
#[macro_use]
mod events;
mod spotify;

pub use events::EventBuildError;
pub use spotify::{Spotify, SpotifyTrack, State};
