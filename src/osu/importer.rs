mod metadata; // rename this
mod timing_data;
mod note_data;

use std::collections::HashMap;

use crate::osu::settings;
use settings::{Settings};

use crate::osu::timing::TimePoint;
use crate::osu::note::NoteData;

pub fn get_settings(sections: &HashMap<String, Vec<String>>) -> Settings {
    let general = metadata::get_general(&sections["[General]"]);
    let editor = metadata::get_editor(&sections["[Editor]"]);
    let metadata = metadata::get_metadata(&sections["[Metadata]"]);
    let difficulty = metadata::get_difficulty(&sections["[Difficulty]"]);
    let events = metadata::get_events(&sections["[Events]"]);

    Settings {
        general,
        editor,
        metadata,
        difficulty,
        events
    }
}

pub fn get_timing_points(sections: &HashMap<String, Vec<String>>) -> Vec<TimePoint>{
    return timing_data::get_timing_points(&sections["[TimingPoints]"]);
}

pub fn get_note_data(sections: &HashMap<String, Vec<String>>) -> NoteData {
    return note_data::get_note_data(&sections["[HitObjects]"]);
}