use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::{prelude::*, BufReader};

mod osu;
use osu::importer;
use osu::Beatmap;

pub fn import(filename: String) -> Result<Beatmap, Error> {
    let reader = open_file(&filename)?;
    let data = get_sections(reader)?;

    let settings = importer::get_settings(&data);
    let timing_data = importer::get_timing_points(&data);
    let note_data = importer::get_note_data(&data);

    return Ok(Beatmap {
        settings,
        timing_data,
        note_data,
    });
}

fn get_sections(reader: BufReader<File>) -> Result<HashMap<String, Vec<String>>, Error> {
    let mut data: HashMap<String, Vec<String>> = HashMap::new();
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

#[cfg(test)]
mod tests {
    #[test]
    fn import_beatmap() {
        let filename = String::from("test_files/beatmap.osu");
        let beatmap = crate::import(filename);

        let beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("|| failed to parse beatmap: {}", e),
        };

        assert_eq!(beatmap.settings.difficulty.approach_rate, 6.9 as f32);
        assert_eq!(beatmap.settings.general.letter_box_in_breaks, false);
        assert_eq!(beatmap.settings.general.samples_match_playback_rate, true);
    }
    #[test]
    fn color_test() {
        let filename = String::from("test_files/ignore/colortest.osu");
        let beatmap = crate::import(filename);

        let beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("|| failed to parse beatmap: {}", e),
        };

        let color = beatmap.settings.colors[0].clone();

        assert_eq!(color.0, 69);
        assert_eq!(color.1, 228);
        assert_eq!(color.2, 13);

        let slider = &beatmap.note_data.sliders[0];
        assert_eq!(slider.x, 47.0);
        assert_eq!(slider.y, 353.0);
        assert_eq!(slider.time, 595.0);
    }
    #[test]
    fn open_blank_beatmap() {
        let filename = String::from("test_files/blank.osu");
        let beatmap = crate::import(filename);

        let _beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("|| failed to parse beatmap: {}", e),
        };
    }

    #[test]
    fn open_broken_beatmap() {
        let filename = String::from("test_files/broken.osu");
        let beatmap = crate::import(filename);

        let _beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("|| failed to parse beatmap: {}", e),
        };
    }
}
