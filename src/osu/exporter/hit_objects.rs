use crate::osu;
use osu::note::{HitObject, Circle, Slider};
use osu::sections::HitObjects;

use std::fmt::Write;
use std::fmt::Error;

pub fn get(h: &HitObjects) -> Result<String, Error> {
    let mut lines = String::new();

    writeln!(&mut lines, "[HitObjects]")?;

    // You have no idea how tired I am of working on this shit

    return Ok(lines)
}

fn write_circle(c: &HitObject<Circle>) -> String{
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

fn write_slider(lines: &mut String, s: &HitObject<Slider>) -> String {
    let edge_sounds = format!(
        "{}|{}",
        s.other.edge_sounds[0],
        s.other.edge_sounds[1]
    );

    let edge_sets = format!(
        "{}|{}",
        s.other.edge_sets[0],
        s.other.edge_sets[1]
    );

    let line = format!(
        "{},{},{},{},{},{},{},{},{},{},{}",
        s.x,
        s.y,
        s.time,
        s.note_type,
        s.hit_sound,
        s.other.params,
        s.other.slides,
        s.other.length,
        edge_sounds,
        edge_sets,
        s.hit_sample
    );

    return line
}