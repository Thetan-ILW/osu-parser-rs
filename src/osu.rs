use std::io::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

mod section;
use section::{General, Metadata};
mod object;
use object::{TimePoint, HitObjects};

pub enum Mode {
    Osu,
    Taiko,
    Fruits,
    Mania,
    Unknown
}

pub struct Beatmap {
    pub general: General,
    pub metadata: Metadata,
    pub timing_points: Vec<TimePoint>,
    pub hit_objects: HitObjects
    //pub difficulty: Difficulty
}

pub fn get_beatmap_from_file(filename: String) -> Result<Beatmap, Error> {
    let reader = import(&filename);
    let reader = match reader {
        Ok(reader) => reader,
        Err(e) => panic!("Failed to read file: {filename} | {e}")
    };

    let data = get_sections(reader)?;
    let general = section::get_general(&data["[General]"]);
    let metadata = section::get_metadata(&data["[Metadata]"]);
    let timing_points = section::get_timing_points(&data["[TimingPoints]"]);
    let hit_objects = section::get_hit_objects(&data["[HitObjects]"]);

    return Ok(Beatmap {
        general,
        metadata,
        timing_points,
        hit_objects
    })
}

fn get_sections(reader: BufReader<File>) -> Result<HashMap<String, Vec<String>>, Error> { 
    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_section: String = String::new();

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => return Err(e)
        };

        if line.len() == 0 {
            continue;
        }

        if is_section_line(&line) {
            current_section = line.clone();
            data.insert(line, vec!());
            continue;
        }

        let section = data.get_mut(&current_section);
        if let Some(section) = section {
            section.push(line);
        }
    }

    return Ok(data)
}

fn is_section_line(line: &String) -> bool {
    let first_char = line.chars().next().unwrap();
    let last_char = line.chars().last().unwrap();

    if first_char == '[' && last_char == ']' {
        return true
    }

    return false
}

fn import(filename: &str) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader)
}