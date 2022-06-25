use crate::osu;
use osu::settings::{Color, Difficulty, Editor, General, Metadata, Events};
use osu::{Mode, OverlayPosition, SampleSet};

// Default values can be found here
// https://osu.ppy.sh/wiki/en/Client/File_formats/Osu_%28file_format%29

impl General {
    pub fn default() -> Self{
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

impl Editor {
    pub fn default() -> Self {
        Editor {
            bookmarks: vec![],
            distance_spacing: 1.0,
            beat_divisor: 16.0,
            grid_size: 32.0,
            timeline_zoom: 1.0,
        }
    }
}

impl Metadata {
    pub fn default() -> Self {
        Metadata {
            title: String::new(),
            title_unicode: String::new(),
            artist: String::new(),
            artist_unicode: String::new(),
            creator: String::new(),
            version: String::new(),
            source: String::new(),
            tags: vec![],
            beatmap_id: -1,
            beatmap_set_id: -1,
        }
    }
}

impl Difficulty {
    pub fn default() -> Self {
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

impl Events {
    pub fn default() -> Self {
        Events {}
    }
}

pub fn get_colors() -> Vec<Color> {
    vec![]
}
