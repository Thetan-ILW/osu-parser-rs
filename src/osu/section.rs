use std::collections::HashMap;
use crate::convert;
use crate::osu::Mode;
use crate::osu::time_point::TimePoint;
use crate::osu::hit_object::{HitSound, HitObject, HitObjects, Circle, Continuous, Slider};

pub struct General {
    pub audio_filename: String,
    pub preview_time: f64,
    pub mode: Mode
}

pub struct Difficulty {
    pub circle_size: f32
}

pub struct Metadata {
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String
}

pub fn get_general(section: &Vec<String>) -> General {
    let mut data: HashMap<String, String> = HashMap::new();

    for line in section {
        get_key_param(line, &mut data)
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

pub fn get_metadata(section: &Vec<String>) -> Metadata {
    let mut data: HashMap<String, String> = HashMap::new();

    for line in section {
        get_key_param(line, &mut data)
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

pub fn get_timing_points(section: &Vec<String>) -> Vec<TimePoint> {
    let mut timing_points: Vec<TimePoint> = vec!();

    for line in section {
        let split = line.split(",");
        let split = split.collect::<Vec<&str>>();

        if split.len() != 8{
            continue; // If line is not valid time point array
        }

        let time_point = TimePoint::from_split(split);
        timing_points.push(time_point);
    }

    return timing_points;
}

pub fn get_hit_objects(section: &Vec<String>) -> HitObjects {
    let mut circles: Vec<HitObject<Circle>> = vec!();
    let mut sliders: Vec<HitObject<Slider>> = vec!();
    let mut continuous: Vec<HitObject<Continuous>> = vec!();

    for line in section {
        let split = line.split(",");
        let split = split.collect::<Vec<&str>>();

        let note_type = convert::to_u8(&split[3]);

        match note_type {
            0b1 | 0b101 => { // Circle | New combo circle | ShortNote
                let circle = Circle::from_split(split);
                circles.push(circle);
            }
            0b10000000 | 0b1000 | 0b1100 => { // Mania hold | Spinner | New combo spinner
                let continuous_object = Continuous::from_split(split);
                continuous.push(continuous_object);
            }
            0b10 | 0b110 => { // slider
                let slider = Slider::from_split(split);
                sliders.push(slider);
            }
            _ => {
                println!("UNKNOWN OBJECT {note_type}");
            }
        };
    }

    let hit_objects = HitObjects {
        circles,
        sliders,
        continuous
    };

    return hit_objects;
}

fn get_key_param(line: &String, data: &mut HashMap<String, String>) {
    let key_param = line.split(":");
    let key_param = key_param.collect::<Vec<&str>>();

    if key_param.len() == 2 {
        data.insert(
            key_param[0].trim().to_string(), 
            key_param[1].trim().to_string()
        );
    }
}

/*
1 - circle (1)
101 - circle with new combo (5)

10 - slider (2)
110 - slider with new combo (6)

1000 - spinner (8)
1100 - spinner with new combo (12)

10000000 - mania hold (128)
*/