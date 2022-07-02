mod info;
mod misc;
mod timing_points;
//mod hit_objects;

use std::fs::File;
use std::io::{prelude::*, LineWriter, Error};

use crate::osu;
use osu::Beatmap;

pub fn write_to_osu(w: &mut LineWriter<File>, beatmap: Beatmap) -> Result<(), Error>{
    const NEW_LINE: &[u8] = "\n".as_bytes();
    let version = "osu file format v14\n";

    let general = info::get_general(&beatmap.info.general).unwrap_or_default();
    let editor = info::get_editor(&beatmap.info.editor).unwrap_or_default();
    let metadata = info::get_metadata(&beatmap.info.metadata).unwrap_or_default();
    let difficulty = info::get_diffuclty(&beatmap.info.difficulty).unwrap_or_default();
    let events = misc::get_events(&beatmap.info.events).unwrap_or_default();
    let timing_points = timing_points::get(&beatmap.timing_points).unwrap_or_default();
    let colors = misc::get_colors(&beatmap.info.colors).unwrap_or_default();
    //let hit_objects = hit_objects::get(&beatmap.hit_objects).unwrap_or_default();

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
    //w.write_all(hit_objects.as_bytes())?;
    return Ok(())
}