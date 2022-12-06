use std::collections::BTreeMap;
use std::fs::File;
use std::io::Error;
use std::io::{prelude::*, BufReader, LineWriter};

pub mod osu;
use osu::importer;
use osu::exporter;
use osu::{Info, Beatmap};
use osu::sections::{TimingPoints, HitObjects};

#[cfg(test)]
mod tests;

/// Only used in conjunction with import_section()
pub enum Section {
    Info(Option<Info>),
    TimingPoints(Option<TimingPoints>),
    HitObjects(Option<HitObjects>)
}

/// Fully import the beatmap
/// ```
/// let beatmap = match osu_parser::import("test_files/beatmap.osu") {
///     Ok(beatmap) => beatmap,
///     Err(error) => panic!("Failed to import beatmap: {}", error)
/// };
/// ```
pub fn import(filename: impl Into<String>) -> Result<Beatmap, Error> {
    let filename = filename.into();

    let reader = open_file(&filename)?;
    let data = get_sections(reader)?;

    let info = importer::get_info(&data);
    let timing_points = importer::get_timing_points(&data);
    let hit_objects = importer::get_hit_objects(&data);

    return Ok(Beatmap {
        info,
        timing_points,
        hit_objects,
    })
}

/// Import of a specific section.
/// Takes a path to an .osu file, and a Section enum.
/// ```
/// let info = osu_parser::import_section("my_beatmap.osu", osu_parser::Section::Info(None));
/// let timing_objects = osu_parser::import_section("my_beatmap.osu", osu_parser::Section::TimingPoints(None));
/// let hit_objects = osu_parser::import_section("my_beatmap.osu", osu_parser::Section::HitObjects(None));
/// ```
pub fn import_section(filename: impl Into<String>, section: Section) -> Result<Section, Error> {
    let filename = filename.into();

    let reader = open_file(&filename)?;
    let data = get_sections(reader)?;

    return Ok(match section {
        Section::Info(_) => Section::Info(Some(importer::get_info(&data))),
        Section::TimingPoints(_) => Section::TimingPoints(Some(importer::get_timing_points(&data))),
        Section::HitObjects(_) => Section::HitObjects(Some(importer::get_hit_objects(&data))),
    })
}

/// Takes a Beatmap struct and writes it to a file.
/// Will be removed in the future, and replaced with Beatmap::to_string(), so that everyone 
/// writes to the file the way they want
pub fn export(path: &str, beatmap: &Beatmap) -> Result<(), Error> {
    let file = File::create(path)?;
    let mut writer = LineWriter::new(file);
    exporter::write_to_osu(&mut writer, beatmap)?;
    writer.flush()?;
    return Ok(())
}

fn get_sections(reader: BufReader<File>) -> Result<BTreeMap<String, Vec<String>>, Error> {
    let mut data: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut current_section: String = String::new();

    for line in reader.lines() {
        let line = line?;

        if line.len() == 0 {
            continue;
        }

        if is_section_line(&line) {
            current_section = line.clone();
            data.insert(line, vec![]);
            continue;
        }

        let section = data.get_mut(&current_section);
        if let Some(section) = section {
            section.push(line);
        }
    }

    return Ok(data);
}

fn is_section_line(line: &String) -> bool {
    let first_char = line.chars().next().unwrap();
    let last_char = line.chars().last().unwrap();

    if first_char == '[' && last_char == ']' {
        return true;
    }

    return false;
}

fn open_file(filename: &str) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader)
}