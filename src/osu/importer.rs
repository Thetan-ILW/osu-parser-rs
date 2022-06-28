mod key_value;
mod info;
mod misc;
mod hit_objects;
mod timing_points;

use std::collections::BTreeMap;

use crate::osu;
use osu::Info;
use osu::sections::{TimingPoints, HitObjects};

pub fn get_info(sections: &BTreeMap<String, Vec<String>>) -> Info {
    let general =       get_section(sections, "[General]");
    let editor =        get_section(sections, "[Editor]");
    let metadata =      get_section(sections, "[Metadata]");
    let difficulty =    get_section(sections, "[Difficulty]");
    let events =        get_section(sections, "[Events]");
    let colors =        get_section(sections, "[Colours]");

    Info {
        general,
        editor,
        metadata,
        difficulty,
        events,
        colors,
    }
}

pub fn get_timing_points(sections: &BTreeMap<String, Vec<String>>) -> TimingPoints {
    return get_section(sections, "[TimingPoints]")
}

pub fn get_hit_objects(sections: &BTreeMap<String, Vec<String>>) -> HitObjects {
    return get_section(sections, "[HitObjects]")
}

fn get_section<T: Import + Default>(sections: &BTreeMap<String, Vec<String>>, name: &str) -> T
{
    return match sections.contains_key(name) { 
        true => T::parse(&sections[name]),
        false => T::default(),
    }
}

pub trait Import {
    fn parse(strings: &Vec<String>) -> Self; // rename this
}