mod key_value;
mod metadata; // rename this
mod misc;
mod note_data;
mod timing_data;

use std::collections::HashMap;

use crate::osu::sections::Settings;

use crate::osu::Import;
use crate::osu::sections::{TimingPoints, HitObjects};

pub fn get_settings(sections: &HashMap<String, Vec<String>>) -> Settings {
    let general =       get_section(sections, "[General]");
    let editor =        get_section(sections, "[Editor]");
    let metadata =      get_section(sections, "[Metadata]");
    let difficulty =    get_section(sections, "[Difficulty]");
    let events =        get_section(sections, "[Events]");
    let colors =        get_section(sections, "[Colours]");

    Settings {
        general,
        editor,
        metadata,
        difficulty,
        events,
        colors,
    }
}

pub fn get_timing_points(sections: &HashMap<String, Vec<String>>) -> TimingPoints {
    return get_section(sections, "[TimingPoints]")
}

pub fn get_hit_objects(sections: &HashMap<String, Vec<String>>) -> HitObjects {
    return get_section(sections, "[HitObjects]")
}

pub fn get_section<T: Import + Default>(sections: &HashMap<String, Vec<String>>, name: &str) -> T
{
    return match sections.contains_key(name) { 
        true => T::parse(&sections[name]),
        false => T::default(),
    }
}