use std::io::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

mod osu;
mod magic;
use osu::Beatmap;
use osu::importer;

pub fn import(filename: String) -> Result<Beatmap, Error> {
    let reader = open_file(&filename);
    let reader = match reader {
        Ok(reader) => reader,
        Err(e) => panic!("Failed to read file: {filename} | {e}")
    };

    let data = get_sections(reader)?;

    let settings = importer::get_settings(&data);
    let timing_data = importer::get_timing_points(&data);
    let note_data = importer::get_note_data(&data);

    return Ok(Beatmap {
        settings,
        timing_data,
        note_data
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

fn open_file(filename: &str) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader)
}