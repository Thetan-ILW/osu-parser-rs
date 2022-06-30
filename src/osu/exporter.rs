mod info;
mod misc;
mod timing_points;
mod hit_objects;

use std::fs::File;
use std::io::{prelude::*, LineWriter};

use crate::osu;
use osu::Beatmap;

pub fn write_to_osu(w: &mut LineWriter<File>, beatmap: Beatmap) {
    const NEW_LINE: &[u8] = "\n".as_bytes();
    let version = "osu file format v14\n";
    let general = info::get_general(&beatmap.info.general);
    let editor = info::get_editor(&beatmap.info.editor);
    let metadata = info::get_metadata(&beatmap.info.metadata);
    let difficulty = info::get_diffuclty(&beatmap.info.difficulty);
    let events = misc::get_events(&beatmap.info.events);
    let timing_points = timing_points::get(&beatmap.timing_points);
    let colors = misc::get_colors(&beatmap.info.colors);
    let hit_objects = hit_objects::get(&beatmap.hit_objects);

    w.write_all(version.as_bytes()).unwrap();
    w.write_all(NEW_LINE).unwrap();
    w.write_all(general.as_bytes()).unwrap();
    w.write_all(NEW_LINE).unwrap();
    w.write_all(editor.as_bytes()).unwrap();
    w.write_all(NEW_LINE).unwrap();
    w.write_all(metadata.as_bytes()).unwrap();
    w.write_all(NEW_LINE).unwrap();
    w.write_all(difficulty.as_bytes()).unwrap();
    w.write_all(NEW_LINE).unwrap();
    w.write_all(events.as_bytes()).unwrap();
    w.write_all(NEW_LINE).unwrap();
    w.write_all(timing_points.as_bytes()).unwrap();
    w.write_all(NEW_LINE).unwrap();
    w.write_all(colors.as_bytes()).unwrap();
    w.write_all(NEW_LINE).unwrap();
    w.write_all(hit_objects.as_bytes()).unwrap();
}