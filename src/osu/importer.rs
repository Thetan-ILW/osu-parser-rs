mod key_value;
mod metadata; // rename this
mod misc;
mod note_data;
mod timing_data;

use std::collections::HashMap;

use crate::osu::settings;
use settings::Settings;

use crate::osu::Import;
use crate::osu::note::NoteData;
use crate::osu::timing::TimePoint;

pub fn get_settings(sections: &HashMap<String, Vec<String>>) -> Settings {
    let general =       get_section(sections, "[General]");
    let editor =        get_section(sections, "[Editor]");
    let metadata =      get_section(sections, "[Metadata]");
    let difficulty =    get_section(sections, "[Difficulty]");
    let events =        get_section(sections, "[Events]");

    let colors = match sections.contains_key("[Colours]") { // not today
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

pub fn get_section<T: Import + Default>(sections: &HashMap<String, Vec<String>>, name: &str) -> T
{
    return match sections.contains_key(name) { 
        true => T::parse(&sections[name]),
        false => T::default(),
    }
}

pub fn get_timing_points(sections: &HashMap<String, Vec<String>>) -> Vec<TimePoint> {
    let time_points = match sections.contains_key("[TimingPoints]") { // not today
        true => timing_data::get_timing_points(&sections["[TimingPoints]"]),
        false => vec![],
    };

    return time_points
}

pub fn get_note_data(sections: &HashMap<String, Vec<String>>) -> NoteData {
    return get_section(sections, "[HitObjects]")
}
