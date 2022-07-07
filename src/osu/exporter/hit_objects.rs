use crate::osu;
use osu::note::{NoteType, HitObject, Circle, Slider, Spinner, Hold};
use osu::sections::HitObjects;

use std::fmt::Write;
use std::fmt::Error;

pub fn get(h: &HitObjects) -> Result<String, Error> {
    let mut lines = String::new();

    writeln!(&mut lines, "[HitObjects]")?;

    let (mut circle_i, mut slider_i, mut spinner_i, mut hold_i) = (0, 0, 0, 0);

    for item in &h.order {
        match item {
            NoteType::Circle  => {
                let circle = get_circle_line(&h.circles[circle_i]);
                writeln!(&mut lines, "{circle}")?;
                circle_i += 1
            }
            NoteType::Slider => {
                let slider = get_slider_line(&h.sliders[slider_i]);
                writeln!(&mut lines, "{slider}")?;
                slider_i += 1;
            }
            NoteType::Spinner => {
                let spinner = get_spinner_line(&h.spinners[spinner_i]);
                writeln!(&mut lines, "{spinner}")?;
                spinner_i += 1;
            }
            NoteType::Hold => {
                let hold = get_hold_line(&h.holds[hold_i]);
                writeln!(&mut lines, "{hold}")?;
                hold_i += 1;
            } 
        }
    }

    return Ok(lines)
}

fn get_circle_line(c: &HitObject<Circle>) -> String{
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

fn get_slider_line(s: &HitObject<Slider>) -> String {
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

fn get_spinner_line(s: &HitObject<Spinner>) -> String {
    let line = format!(
        "{},{},{},{},{},{},{}",
        s.x,
        s.y,
        s.time,
        s.note_type,
        s.hit_sound,
        s.other.end_time,
        s.hit_sample
    );

    return line
}

fn get_hold_line(h: &HitObject<Hold>) -> String {
    let line = format!(
        "{},{},{},{},{},{}:{}",
        h.x,
        h.y,
        h.time,
        h.note_type,
        h.hit_sound,
        h.other.end_time,
        h.hit_sample
    );

    return line
}