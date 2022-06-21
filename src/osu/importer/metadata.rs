use std::collections::HashMap;
use crate::convert;
use crate::osu::Mode;
use crate::osu::settings::{
    General, Editor, Metadata, Difficulty, Events
};

pub fn get_general(section: &Vec<String>) -> General {
    let mut data: HashMap<String, String> = HashMap::new();

    for line in section {
        get_key_value(line, &mut data)
    }

    let audio_filename = String::new();//data["AudioFilename"].clone();
    let preview_time = convert::to_f64(&data["PreviewTime"]);
    let mode = convert::to_u32(&data["Mode"]);

    let mode: Mode = match mode {
        0 => Mode::Osu,
        1 => Mode::Taiko,
        2 => Mode::Fruits,
        3 => Mode::Mania,
        _ => Mode::Unknown
    };

    General{
        audio_filename,
        preview_time,
        mode
    }
}

pub fn get_editor(section: &Vec<String>) -> Editor {
    Editor {}
}

pub fn get_difficulty(section: &Vec<String>) -> Difficulty {
    let mut data: HashMap<String, String> = HashMap::new();
    for line in section {
        get_key_value(line, &mut data)
    }

    let circle_size = convert::to_f32(&data["CircleSize"]);
    
    Difficulty {
        circle_size
    }
}

pub fn get_metadata(section: &Vec<String>) -> Metadata {
    let mut data: HashMap<String, String> = HashMap::new();

    for line in section {
        get_key_value(line, &mut data)
    }

    let title =             data["Title"].clone();
    let title_unicode =     data["TitleUnicode"].clone();
    let artist =            data["Artist"].clone();
    let artist_unicode =    data["ArtistUnicode"].clone();
    let creator =           data["Creator"].clone();
    let version =           data["Version"].clone();

    Metadata{
        title,
        title_unicode,
        artist,
        artist_unicode,
        creator,
        version
    }
}

pub fn get_events(section: &Vec<String>) -> Events
{
    Events {}
}

fn get_key_value(line: &String, data: &mut HashMap<String, String>) {
    let key_value = line.split(":");
    let key_value = key_value.collect::<Vec<&str>>();

    if key_value.len() == 2 {
        data.insert(
            key_value[0].trim().to_string(), 
            key_value[1].trim().to_string()
        );
    }
}