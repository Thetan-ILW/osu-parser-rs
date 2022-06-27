mod info;
mod misc;

use std::fs::File;
use std::io::{prelude::*, LineWriter};

use crate::osu;
use osu::Beatmap;

pub fn write_to_osu(writer: &mut LineWriter<File>, beatmap: Beatmap) {
    const NEW_LINE: &[u8] = "\n".as_bytes();
    let version = "osu file format v128";
    let editor = info::get_editor(&beatmap.info.editor);
    let metadata = info::get_metadata(&beatmap.info.metadata);
    let difficulty = info::get_diffuclty(&beatmap.info.difficulty);
    let colors = misc::get_colors(&beatmap.info.colors);
    writer.write_all(version.as_bytes());
    writer.write_all(NEW_LINE);
    writer.write_all(NEW_LINE);
    writer.write_all(editor.as_bytes());
    writer.write_all(NEW_LINE);
    writer.write_all(metadata.as_bytes());
    writer.write_all(NEW_LINE);
    writer.write_all(difficulty.as_bytes());
    writer.write_all(NEW_LINE);
    writer.write_all(colors.as_bytes());
}