use crate::osu::Mode;

pub struct Settings {
    pub general: General,
    pub editor: Editor,
    pub metadata: Metadata,
    pub difficulty: Difficulty,
    pub events: Events
}

pub struct General {
    pub audio_filename: String,
    pub preview_time: f64,
    pub mode: Mode
}

pub struct Editor {}

pub struct Metadata {
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String
}

pub struct Difficulty {
    pub circle_size: f32
}

pub struct Events {}

