use crate::osu;
use osu::note::{Additions, HitObject, Slider, Spinner, Hold};
use osu::sections::HitObjects;

use std::fmt::Write;
use std::fmt::Error;

pub fn get(h: &HitObjects) -> Result<String, Error> {
    let mut lines = String::new();

    writeln!(&mut lines, "[HitObjects]")?;

    for item in &h.data {
        match &item.additions {
            Additions::None => {
                panic!("lol")
            },
            Additions::Circle(_)  => {
                let circle = get_circle_line(item);
                writeln!(&mut lines, "{circle}")?;
            }
            Additions::Slider(other) => {
                let slider = get_slider_line(item, other);
                writeln!(&mut lines, "{slider}")?;

            }
            Additions::Spinner(other) => {
                let spinner = get_spinner_line(item, other);
                writeln!(&mut lines, "{spinner}")?;
            }
            Additions::Hold(other) => {
                let hold = get_hold_line(item, other);
                writeln!(&mut lines, "{hold}")?;
            } 
        }
    }

    return Ok(lines)
}

fn get_circle_line(c: &HitObject) -> String{
    let line = format!(
        "{},{},{},{},{},{}",
        c.x,
        c.y,
        c.time,
        c.note_type,
        c.hit_sound,
        c.hit_sample
    );

    return line
}

fn get_slider_line(s: &HitObject, other: &Slider) -> String {
    let edge_sounds = format!(
        "{}|{}",
        other.edge_sounds[0],
        other.edge_sounds[1]
    );

    let edge_sets = format!(
        "{}|{}",
        other.edge_sets[0],
        other.edge_sets[1]
    );

    let line = format!(
        "{},{},{},{},{},{},{},{},{},{},{}",
        s.x,
        s.y,
        s.time,
        s.note_type,
        s.hit_sound,
        other.params,
        other.slides,
        other.length,
        edge_sounds,
        edge_sets,
        s.hit_sample
    );

    return line
}

fn get_spinner_line(s: &HitObject, other: &Spinner) -> String {
    let line = format!(
        "{},{},{},{},{},{},{}",
        s.x,
        s.y,
        s.time,
        s.note_type,
        s.hit_sound,
        other.end_time,
        s.hit_sample
    );

    return line
}

fn get_hold_line(h: &HitObject, other: &Hold) -> String {
    let line = format!(
        "{},{},{},{},{},{}:{}",
        h.x,
        h.y,
        h.time,
        h.note_type,
        h.hit_sound,
        other.end_time,
        h.hit_sample
    );

    return line
}