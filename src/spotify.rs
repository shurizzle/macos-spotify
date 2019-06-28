use crate::events::{
    AEDesc, AutoPropertyType, EventEnum, EventPropertyReader, EventedObject, EventedRootObject,
    EventedSubObject, ResType
};
use libc::c_char;
use std::io::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum State {
    STOPPED = 0x6b505353,
    PLAYING = 0x6b505350,
    PAUSED = 0x6b505370,
}

impl Into<u32> for State {
    fn into(self) -> u32 {
        self as u32
    }
}

impl EventEnum for State {
    fn from_int(value: u32) -> Self {
        match value {
            0x6b505353 => State::STOPPED,
            0x6b505350 => State::PLAYING,
            0x6b505370 => State::PAUSED,
            _ => panic!("Invalid enum value"),
        }
    }
}

impl AutoPropertyType for State {
    fn read(reader: EventPropertyReader) -> Result<Option<State>> {
        EventEnum::read(reader)
    }

    fn to_desc(&self) -> Result<AEDesc> {
        EventEnum::to_desc(self)
    }
}

pub const SIGNATURE: ResType = res_type!("spfy");
pub const BUNDLE_ID: *const c_char = b"com.spotify.client\0" as *const u8 as *const c_char;
pub const EVENT_ID_PLAYPAUSE: ResType = res_type!("PlPs");
pub const EVENT_ID_PLAY: ResType = res_type!("Play");
pub const EVENT_ID_PAUSE: ResType = res_type!("Paus");
pub const EVENT_ID_NEXT: ResType = res_type!("Next");
pub const EVENT_ID_PREVIOUS: ResType = res_type!("Prev");
pub const EVENT_ID_PLAY_TRACK: ResType = res_type!("PCtx");

pub const KEY_CONTEXT: ResType = res_type!("cotx");

pub const PROPERTY_STATE: ResType = res_type!("pPlS");
pub const PROPERTY_SHUFFLING: ResType = res_type!("pShu");
pub const PROPERTY_REPEATING: ResType = res_type!("pRep");
pub const PROPERTY_POSITION: ResType = res_type!("pPos");
pub const PROPERTY_VOLUME: ResType = res_type!("pVol");
pub const PROPERTY_TRACK: ResType = res_type!("pTrk");
pub const PROPERTY_ARTIST: ResType = res_type!("pArt");
pub const PROPERTY_ID: ResType = res_type!("ID  ");
pub const PROPERTY_NAME: ResType = res_type!("pnam");
pub const PROPERTY_ALBUM: ResType = res_type!("pAlb");
pub const PROPERTY_ALBUM_ARTIST: ResType = res_type!("pAlA");
// pub const PROPERTY_ARTWORK: ResType = res_type!("tAwk");
pub const PROPERTY_ARTWORK_URL: ResType = res_type!("aUrl");
pub const PROPERTY_DISK_NUMBER: ResType = res_type!("pDsN");
pub const PROPERTY_DURATION: ResType = res_type!("pDur");
pub const PROPERTY_PLAYED_COUNT: ResType = res_type!("pPlC");
pub const PROPERTY_POPULARITY: ResType = res_type!("spPo");
pub const PROPERTY_SPOTIFY_URL: ResType = res_type!("spur");
pub const PROPERTY_STARRED: ResType = res_type!("spSt");
pub const PROPERTY_TRACK_NUMBER: ResType = res_type!("pTrN");

pub struct SpotifyTrack {
    signature: ResType,
    bundle_id: *const c_char,
    target_object: AEDesc,
}

impl SpotifyTrack {
    pub fn artist(&self) -> Result<Option<String>> {
        self.get_property(PROPERTY_ARTIST)
    }

    pub fn id(&self) -> Result<Option<String>> {
        self.get_property(PROPERTY_ID)
    }

    pub fn name(&self) -> Result<Option<String>> {
        self.get_property(PROPERTY_NAME)
    }

    pub fn album(&self) -> Result<Option<String>> {
        self.get_property(PROPERTY_ALBUM)
    }

    pub fn album_artist(&self) -> Result<Option<String>> {
        self.get_property(PROPERTY_ALBUM_ARTIST)
    }

    pub fn artwork_url(&self) -> Result<Option<String>> {
        self.get_property(PROPERTY_ARTWORK_URL)
    }

    pub fn disk_number(&self) -> Result<Option<i32>> {
        self.get_property(PROPERTY_DISK_NUMBER)
    }

    pub fn duration(&self) -> Result<Option<i32>> {
        self.get_property(PROPERTY_DURATION)
    }

    pub fn played_count(&self) -> Result<Option<i32>> {
        self.get_property(PROPERTY_PLAYED_COUNT)
    }

    pub fn popularity(&self) -> Result<Option<i32>> {
        self.get_property(PROPERTY_POPULARITY)
    }

    pub fn spotify_url(&self) -> Result<Option<String>> {
        self.get_property(PROPERTY_SPOTIFY_URL)
    }

    pub fn url(&self) -> Result<Option<String>> {
        if let Some(url) = self.spotify_url()? {
            Ok(Some(format!(
                "https://open.spotify.com/track/{}",
                &url[14..]
            )))
        } else {
            Ok(None)
        }
    }

    pub fn starred(&self) -> Result<Option<bool>> {
        self.get_property(PROPERTY_STARRED)
    }

    pub fn track_number(&self) -> Result<Option<i32>> {
        self.get_property(PROPERTY_TRACK_NUMBER)
    }

    // pub fn artwork(&self) -> Result<Option<Vec<u8>>> {
    //     self.get_property(PROPERTY_ARTWORK)
    // }
}

impl EventedObject for SpotifyTrack {
    fn signature(&self) -> ResType {
        self.signature
    }

    fn bundle_id(&self) -> *const c_char {
        self.bundle_id
    }

    fn target_object(&self) -> &AEDesc {
        &self.target_object
    }
}

impl AutoPropertyType for SpotifyTrack {
    fn read(reader: EventPropertyReader) -> Result<Option<SpotifyTrack>> {
        EventedSubObject::read(reader)
    }
}

impl EventedSubObject for SpotifyTrack {
    fn instantiate(
        signature: ResType,
        bundle_id: *const c_char,
        target_object: AEDesc,
    ) -> SpotifyTrack {
        SpotifyTrack {
            signature,
            bundle_id,
            target_object,
        }
    }
}

pub struct Spotify {
    target_object: AEDesc,
}

impl EventedRootObject for Spotify {}

impl EventedObject for Spotify {
    fn signature(&self) -> ResType {
        SIGNATURE
    }

    fn bundle_id(&self) -> *const c_char {
        BUNDLE_ID
    }

    fn target_object(&self) -> &AEDesc {
        &self.target_object
    }
}

impl Spotify {
    pub fn new() -> Spotify {
        Spotify {
            target_object: Default::default(),
        }
    }

    pub fn play_pause(&self) -> Result<()> {
        call!(self, EVENT_ID_PLAYPAUSE)
    }

    pub fn play(&self) -> Result<()> {
        call!(self, EVENT_ID_PLAY)
    }

    pub fn pause(&self) -> Result<()> {
        call!(self, EVENT_ID_PAUSE)
    }

    pub fn next(&self) -> Result<()> {
        call!(self, EVENT_ID_NEXT)
    }

    pub fn previous(&self) -> Result<()> {
        call!(self, EVENT_ID_PREVIOUS)
    }

    pub fn prev(&self) -> Result<()> {
        self.previous()
    }

    pub fn state(&self) -> Result<Option<State>> {
        self.get_property(PROPERTY_STATE)
    }

    pub fn is_shuffling(&self) -> Result<Option<bool>> {
        self.get_property(PROPERTY_SHUFFLING)
    }

    pub fn set_shuffling(&self, is_it: bool) -> Result<()> {
        self.set_property(PROPERTY_SHUFFLING, &is_it)
    }

    pub fn is_repeating(&self) -> Result<Option<bool>> {
        self.get_property(PROPERTY_REPEATING)
    }

    pub fn set_repeating(&self, is_it: bool) -> Result<()> {
        self.set_property(PROPERTY_REPEATING, &is_it)
    }

    pub fn position(&self) -> Result<Option<f64>> {
        self.get_property(PROPERTY_POSITION)
    }

    pub fn set_position(&self, pos: f64) -> Result<()> {
        self.set_property(PROPERTY_POSITION, &pos)
    }

    pub fn set_pos(&self, pos: f64) -> Result<()> {
        self.set_position(pos)
    }

    pub fn pos(&self) -> Result<Option<f64>> {
        self.position()
    }

    pub fn volume(&self) -> Result<Option<i32>> {
        self.get_property(PROPERTY_VOLUME)
    }

    pub fn set_volume(&self, vol: i32) -> Result<()> {
        self.set_property(PROPERTY_VOLUME, &vol)
    }

    pub fn track(&self) -> Result<Option<SpotifyTrack>> {
        self.get_property(PROPERTY_TRACK)
    }

    pub fn play_track(&self, track: String, context: Option<String>) -> Result<()> {
        if let Some(context) = context {
            call!(self, EVENT_ID_PLAY_TRACK, &track, KEY_CONTEXT: &context)
        } else {
            call!(self, EVENT_ID_PLAY_TRACK, &track)
        }
    }
}
