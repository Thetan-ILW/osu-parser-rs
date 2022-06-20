use std::collections::HashMap;
use crate::osu::Mode;
use crate::osu::object::{TimePoint, SampleSet, HitSound};
use crate::osu::object::{HitObject, HitObjects, Circle, Continuous, Slider};

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
    let preview_time = to_f64(&data["PreviewTime"]);
    let mode = to_u32(&data["Mode"]);

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
            continue; // If line is not time point array
        }

        let time = to_f64(split[0]);
        let beat_length = to_f64(split[1]);
        let meter = to_u32(split[2]);

        let sample_set = to_u32(split[3]);
        let sample_set = match sample_set {
            0 => SampleSet::Default,
            1 => SampleSet::Normal,
            2 => SampleSet::Soft,
            3 => SampleSet::Drum,
            _ => SampleSet::Default
        };

        let sample_index = to_u32(split[4]);
        let volume = to_f32(split[5]);
        let uninherited = to_bool(split[6]); 
        let effects = to_u8(split[7]);

        timing_points.push(TimePoint{
            time,
            beat_length,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited,
            effects
        })
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

        let x =         to_f32(&split[0]);
        let y =         to_f32(&split[1]);
        let time =      to_f64(&split[2]);
        let note_type = to_u8(&split[3]);
        let hit_sound: HitSound = HitSound::new(to_u8(&split[4]));

        match note_type {
            0b1 | 0b101 => { // Circle | New combo circle | ShortNote
                let hit_sample = split[5].to_string();
                let circle = HitObject::<Circle> {
                    x, y, time,
                    note_type,
                    hit_sound,
                    hit_sample,
                    other: Circle {}
                };

                circles.push(circle);
            }
            0b10000000 | 0b1000 | 0b1100 => { // Mania hold | Spinner | New combo spinner
                // here we split end_time:hit_sample:hit_sample:hit_sample:hit_sample
                // to end_time and hit_sample
                let line_end_split = split[5].split(":");
                let mut line_end_split = line_end_split.collect::<Vec<&str>>();
                let end_time = to_f64(line_end_split[0]);
                line_end_split.remove(0);
                let mut hit_sample = String::new();
                for element in split {
                    hit_sample.push_str(element);
                }

                let continuous_object = HitObject::<Continuous> {
                    x, y, time,
                    note_type,
                    hit_sound,
                    hit_sample,
                    other: Continuous {
                        end_time,
                    }
                };

                continuous.push(continuous_object);
            }
            0b10 | 0b110 => { // slider
                let params = split[5].to_string();
                let slides = to_u32(&split[6]);
                let length = to_f64(&split[7]);
                
                let edge_sounds: [HitSound; 2];
                let edge_sets: [String; 2];

                if split.len() == 11 {
                    let help_me = split[8].split("|");
                    let help_me = help_me.collect::<Vec<&str>>();
                    edge_sounds = [
                        HitSound::new(to_u8(help_me[0])),
                        HitSound::new(to_u8(help_me[1])),
                    ];
    
                    let help_me = split[9].split("|");
                    let help_me = help_me.collect::<Vec<&str>>();
                    edge_sets = [
                        help_me[0].to_string(),
                        help_me[1].to_string()
                    ];
                }
                else {
                    edge_sounds = [HitSound::Normal, HitSound::Normal];
                    edge_sets = ["0:0".to_string(), "0:0".to_string()];
                }

                let hit_sample = split[10].to_string();

                let slider = HitObject::<Slider> { 
                    x, y, time,
                    note_type,
                    hit_sound,
                    hit_sample,
                    other: Slider {
                        params,
                        slides,
                        length,
                        edge_sounds,
                        edge_sets
                    }
                };

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
// ABSOLUTLY NOTHING BELOW!!!
fn to_bool(string: &str) -> bool {
    return match string.parse::<i8>() {
        Ok(value) => match value{
            0 => false,
            1 => true,
            _ => false
        },
        Err(_) => {
            println!("FAILED TO CONVERT VALUE TO BOOL: {string}");
            false
        }
    };
}

fn to_u8(string: &str) -> u8 {
    return match string.parse::<u8>() {
        Ok(value) => value,
        Err(_) => {
            println!("FAILED TO CONVERT VALUE TO u8: {string}");
            0
        }
    };
}

fn to_u32(string: &str) -> u32 {
    return match string.parse::<u32>() {
        Ok(value) => value,
        Err(_) => {
            println!("FAILED TO CONVERT VALUE TO u32: {string}");
            0
        }
    };
}

fn to_f32(string: &str) -> f32 {
    return match string.parse::<f32>() {
        Ok(value) => value,
        Err(_) => {
            println!("FAILED TO CONVERT VALUE TO f32: {string}");
            0.0
        }
    };
}

fn to_f64(string: &str) -> f64 {
    return match string.parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("FAILED TO CONVERT VALUE TO f32: {string}");
            0.0
        }
    };
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