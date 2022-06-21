use crate::osu::{Mode, SampleSet};

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
    pub countdown: bool,
    pub sample_set: SampleSet,
    pub stack_leniency: f32,
    pub mode: Mode,
    pub letter_box_in_breaks: bool,
    pub widescreen_storyboard: bool,
    pub samples_match_playback: bool
}

pub struct Editor {
    pub distance_spacing: f32,
    pub beat_divisor: f32,
    pub grid_size: f32,
    pub timeline_zoom: f32
}

pub struct Metadata {
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String
}

pub struct Difficulty {
    pub hp_drain_rate: f32,
    pub circle_size: f32,
    pub overall_difficulty: f32,
    pub approach_rate: f32,
    pub slider_multiplier: f32,
    pub slider_tick_rate: f32
}

pub struct Events {}

