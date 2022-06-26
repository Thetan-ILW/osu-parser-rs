// This is a module that stores section structures. 
// Logic to these structures is implemented in other modules
use crate::osu;
use osu::{Mode, OverlayPosition, SampleSet, Color};
use osu::timing::TimePoint;
use osu::note::{HitObject, Circle, Slider, Spinner, Hold};

pub struct General {
    pub audio_filename: String,
    pub audio_lead_in: f64,
    pub preview_time: f64,
    pub countdown: u32,
    pub sample_set: SampleSet,
    pub stack_leniency: f64,
    pub mode: Mode,
    pub letter_box_in_breaks: bool,
    pub use_skin_sprites: bool,
    pub overlay_position: OverlayPosition,
    pub skin_preference: String,
    pub epilepsy_warning: bool,
    pub countdown_offset: u32,
    pub special_style: bool,
    pub widescreen_storyboard: bool,
    pub samples_match_playback_rate: bool,
}

impl Default for General {
    fn default() -> Self{
        General {
            audio_filename: String::new(),
            audio_lead_in: 0.0,
            preview_time: -1.0,
            countdown: 1,
            sample_set: SampleSet::Normal,
            stack_leniency: 0.7,
            mode: Mode::Osu,
            letter_box_in_breaks: false,
            use_skin_sprites: false,
            overlay_position: OverlayPosition::NoChange,
            skin_preference: String::new(),
            epilepsy_warning: false,
            countdown_offset: 0,
            special_style: false,
            widescreen_storyboard: false,
            samples_match_playback_rate: false,
        }
    }
}

#[derive(Default)]
pub struct Editor {
    pub bookmarks: Vec<f64>,
    pub distance_spacing: f32,
    pub beat_divisor: f32,
    pub grid_size: f32,
    pub timeline_zoom: f32,
}

#[derive(Default)]
pub struct Metadata {
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String,
    pub source: String,
    pub tags: Vec<String>,
    pub beatmap_id: i32,
    pub beatmap_set_id: i32,
}

pub struct Difficulty {
    pub hp_drain_rate: f32,
    pub circle_size: f32,
    pub overall_difficulty: f32,
    pub approach_rate: f32,
    pub slider_multiplier: f32,
    pub slider_tick_rate: f32,
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty {
            hp_drain_rate: 5.0,
            circle_size: 5.0,
            overall_difficulty: 5.0,
            approach_rate: 5.0,
            slider_multiplier: 1.0,
            slider_tick_rate: 1.0,
        }
    }
}

#[derive(Default)]
pub struct Events {}

#[derive(Default)]
pub struct TimingPoints {
    pub data: Vec<TimePoint>
}

#[derive(Default)]
pub struct Colors {
    pub data: Vec<Color>
}

#[derive(Default)]
pub struct HitObjects {
    pub circles: Vec<HitObject<Circle>>,
    pub sliders: Vec<HitObject<Slider>>,
    pub spinners: Vec<HitObject<Spinner>>,
    pub holds: Vec<HitObject<Hold>>,
}
