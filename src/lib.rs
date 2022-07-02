use std::collections::BTreeMap;
use std::fs::File;
use std::io::Error;
use std::io::{prelude::*, BufReader, LineWriter};

mod osu;
use osu::importer;
use osu::exporter;
use osu::{Info, Beatmap};
use osu::sections::{TimingPoints, HitObjects};

pub fn import(filename: String) -> Result<Beatmap, Error> {
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

// It looks terrible
pub fn import_info(filename: String) -> Result<Info, Error> {
    let reader = open_file(&filename)?;
    let data = get_sections(reader)?;
    return Ok(importer::get_info(&data));
}

pub fn import_timing_points(filename: String) -> Result<TimingPoints, Error> {
    let reader = open_file(&filename)?;
    let data = get_sections(reader)?;
    return Ok(importer::get_timing_points(&data))
}

pub fn import_hit_objects(filename: String) -> Result<HitObjects, Error> {
    let reader = open_file(&filename)?;
    let data = get_sections(reader)?;
    return Ok(importer::get_hit_objects(&data))
}

pub fn export(path: &str, beatmap: Beatmap) -> Result<(), Error> {
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