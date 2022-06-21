mod metadata; // rename this
mod timing_data;
mod note_data;

use std::collections::HashMap;

use crate::convert;

use crate::osu::settings;
use settings::{Settings};

use crate::osu::timing::TimePoint;

use crate::osu::note::{
    NoteData, 
    HitObject, 
    Circle, Slider, Continuous 
};

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

pub fn get_notedata(sections: &HashMap<String, Vec<String>>) -> NoteData {
    let section = &sections["[HitObjects]"];

    let mut circles: Vec<HitObject<Circle>> = vec!();
    let mut sliders: Vec<HitObject<Slider>> = vec!();
    let mut continuous: Vec<HitObject<Continuous>> = vec!();

    for line in section {
        let split = line.split(",");
        let split = split.collect::<Vec<&str>>();

        let note_type = convert::to_u8(&split[3]);

        match note_type {
            0b1 | 0b101 => { // Circle | New combo circle | ShortNote
                let circle = Circle::from_split(split);
                circles.push(circle);
            }
            0b10000000 | 0b1000 | 0b1100 => { // Mania hold | Spinner | New combo spinner
                let continuous_object = Continuous::from_split(split);
                continuous.push(continuous_object);
            }
            0b10 | 0b110 => { // slider
                let slider = Slider::from_split(split);
                sliders.push(slider);
            }
            _ => {
                println!("UNKNOWN OBJECT {note_type}");
            }
        };
    }

    NoteData {
        circles,
        sliders,
        continuous
    }
}