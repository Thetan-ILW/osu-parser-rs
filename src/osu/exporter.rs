mod info;
mod misc;
mod timing_points;
mod hit_objects;

use std::fs::File;
use std::io::{prelude::*, LineWriter, Error};

use crate::osu;
use osu::Beatmap;

use super::sections::DefaultExport;

pub fn write_to_osu(w: &mut LineWriter<File>, beatmap: Beatmap) -> Result<(), Error>{
    const NEW_LINE: &[u8] = b"\n";
    let version = "osu file format v14\n";

    let general = get_section_as_string(&beatmap.info.general);
    let editor = get_section_as_string(&beatmap.info.editor);
    let metadata = get_section_as_string(&beatmap.info.metadata);
    let difficulty = get_section_as_string(&beatmap.info.difficulty);
    let events = get_section_as_string(&beatmap.info.events);
    let timing_points = get_section_as_string(&beatmap.timing_points);
    let colors = get_section_as_string(&beatmap.info.colors);
    let hit_objects = get_section_as_string(&beatmap.hit_objects);

    w.write_all(version.as_bytes())?;
    w.write_all(NEW_LINE)?;
    w.write_all(general.as_bytes())?;
    w.write_all(NEW_LINE)?;
    w.write_all(editor.as_bytes())?;
    w.write_all(NEW_LINE)?;
    w.write_all(metadata.as_bytes())?;
    w.write_all(NEW_LINE)?;
    w.write_all(difficulty.as_bytes())?;
    w.write_all(NEW_LINE)?;
    w.write_all(events.as_bytes())?;
    w.write_all(NEW_LINE)?;
    w.write_all(timing_points.as_bytes())?;
    w.write_all(NEW_LINE)?;
    w.write_all(colors.as_bytes())?;
    w.write_all(NEW_LINE)?;
    w.write_all(hit_objects.as_bytes())?;
    return Ok(())
}

fn get_section_as_string<T: Export + DefaultExport>(section: &T) -> String {
    let lines = section.as_string();

    match lines {
        Ok(l) => return l,
        Err(e) => return {
            println!("Failed to export section: {e}");
            T::default_export()
        }
    }
}

pub trait Export {
    fn as_string(&self) -> Result<String, std::fmt::Error>;
}