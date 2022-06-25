mod key_value;
mod metadata; // rename this
mod misc;
mod note_data;
mod timing_data;

use std::collections::HashMap;

use crate::osu::settings;
use settings::{Difficulty, Editor, General, Metadata, Events};
use settings::Settings;

use crate::osu::note::NoteData;
use crate::osu::timing::TimePoint;

pub fn get_settings(sections: &HashMap<String, Vec<String>>) -> Settings {
    // I can't think of a solution in 32 degree heat
    let general = match sections.contains_key("[General]") {
        true => metadata::get_general(&sections["[General]"]),
        false => General::default(),
    };

    let editor = match sections.contains_key("[Editor]") {
        true => metadata::get_editor(&sections["[Editor]"]),
        false => Editor::default(),
    };

    let metadata = match sections.contains_key("[Metadata]") {
        true => metadata::get_metadata(&sections["[Metadata]"]),
        false => Metadata::default(),
    };

    let difficulty = match sections.contains_key("[Difficulty]") {
        true => metadata::get_difficulty(&sections["[Difficulty]"]),
        false => Difficulty::default(),
    };

    let events = match sections.contains_key("[Events]") {
        true => misc::get_events(&sections["[Events]"]),
        false => Events::default(),
    };

    let colors = match sections.contains_key("[Colours]") {
        true => misc::get_colors(&sections["[Colours]"]),
        false => vec![],
    };

    Settings {
        general,
        editor,
        metadata,
        difficulty,
        events,
        colors,
    }
}

pub fn get_timing_points(sections: &HashMap<String, Vec<String>>) -> Vec<TimePoint> {
    let time_points = match sections.contains_key("[TimingPoints]") {
        true => timing_data::get_timing_points(&sections["[TimingPoints]"]),
        false => vec![],
    };

    return time_points
}

pub fn get_note_data(sections: &HashMap<String, Vec<String>>) -> NoteData {
    let hit_objects = match sections.contains_key("[HitObjects]") {
        true => note_data::get_note_data(&sections["[HitObjects]"]),
        false => NoteData::default()
    };

    return hit_objects
}
