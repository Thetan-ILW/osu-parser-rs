use crate::osu;
use osu::note::{HitObject, Circle, Slider};
use osu::sections::HitObjects;

use std::fmt::Write;

pub fn get(h: &HitObjects) -> String {
    let mut lines = String::new();

    if let Err(e) = writeln!(&mut lines, "[HitObjects]") {
        println!("{e}");
        return lines
    }

    // You have no idea how tired I am of working on this shit

    return lines
}

fn write_circle(lines: &mut String, c: &HitObject<Circle>) {
    let line = format!(
        "{},{},{},{},{},{}",
        c.x,
        c.y,
        c.time,
        c.note_type,
        c.hit_sound,
        c.hit_sample
    );

    writeln!(lines, "{line}").unwrap();
}

fn write_slider(lines: &mut String, s: &HitObject<Slider>) {
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

    writeln!(lines, "{line}").unwrap();
}