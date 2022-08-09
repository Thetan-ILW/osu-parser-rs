use std::ops::Not;

use crate::osu;
use osu::importer::Import;
use osu::sections::HitObjects;
use osu::note::{NoteType, HitObject, Circle, Slider, Spinner, Hold};

impl Import for HitObjects {
    fn parse(section: &Vec<String>) -> Self {
        let mut data: Vec<HitObject> = vec![];
    
        for line in section {
            let split = line.split(",");
            let split = split.collect::<Vec<&str>>();
    
            let note_type: u8 = split[3].parse().unwrap_or_else(|_|0);
            // https://github.com/nojhamster/osu-parser/blob/master/index.js
            // stole the code from there ^^^, because i'm DUMB 🤪 to think for myself
            if (1 & note_type) == 1 {
                let circle = Circle::from_split(split);
                data.push(circle);
            }
            else if (2 & note_type) == 2 {
                let slider = Slider::from_split(split);
                data.push(slider);
            }
            else if (8 & note_type) == 8 {
                let spinner = Spinner::from_split(split);
                data.push(spinner);
            }
            else if (128 & note_type) == 128 {
                let hold = Hold::from_split(split);
                data.push(hold);
            }
            else {
                println!("😡 UNKNOWN OBJECT {note_type}");
            }
        }
    
        HitObjects {
            data
        }
    }
}

impl HitObject {
    pub fn from_split(split: &Vec<&str>) -> Self {
        let x = split[0].parse().unwrap_or_else(|_| 0.0);
        let y = split[1].parse().unwrap_or_else(|_| 0.0);
        let time = split[2].parse().unwrap_or_else(|_| 0.0);
        let note_type = split[3].parse().unwrap_or_else(|_| 0);
        let hit_sound = split[4].parse().unwrap_or_else(|_| 0);
        let hit_sample = String::new();
        let other = NoteType::None;

        Self {
            x,
            y,
            time,
            note_type,
            hit_sound,
            hit_sample,
            other,
        }
    }
}

impl Circle {
    pub fn from_split(split: Vec<&str>) -> HitObject {
        let mut circle = HitObject::from_split(&split);

        circle.hit_sample = split[5].to_string();
        circle.other = NoteType::Circle( Circle {});

        return circle;
    }
}

impl Spinner {
    pub fn from_split(split: Vec<&str>) -> HitObject {
        let end_time = split[5].parse().unwrap_or_else(|_| 0.0);
        let mut spinner = HitObject::from_split(&split);

        spinner.hit_sample = split[6].to_string();
        spinner.other = NoteType::Spinner( Spinner { end_time });

        return spinner;
    }
}

impl Slider {
    pub fn from_split(split: Vec<&str>) -> HitObject {
        let params = split[5].to_string();
        let slides = split[6].parse().unwrap_or_else(|_| 0);
        let length = split[7].parse().unwrap_or_else(|_| 0.0);
        let mut edge_sounds: [u8; 2] = [0, 0];
        let mut edge_sets: [String; 2] = ["0:0".to_string(), "0:0".to_string()];
        let hit_sample: String;

        // slider can have different number of parameters
        // garbage from below seems to work correctly
        match split.len() {
            8 => {
                hit_sample = String::from("");
            }
            9 => {
                hit_sample = split[8].to_string();
            }
            11 => {
                let edge_sounds_split = split[8].split("|");
                let edge_sounds_split = edge_sounds_split.collect::<Vec<&str>>();
                edge_sounds = [
                    edge_sounds_split[0].parse().unwrap_or_else(|_| 0),
                    edge_sounds_split[1].parse().unwrap_or_else(|_| 0),
                ];

                let edge_sets_split = split[9].split("|");
                let help_me = edge_sets_split.collect::<Vec<&str>>();
                edge_sets = [help_me[0].to_string(), help_me[1].to_string()];

                hit_sample = split[10].to_string();
            }
            _ => {
                hit_sample = String::from("");
                println!("slider {} split.len is {}", split[2], split.len())
            }
        }

        let mut slider = HitObject::from_split(&split,);

        slider.hit_sample = hit_sample;
        slider.other = NoteType::Slider(Slider {
            params,
            slides,
            length,
            edge_sounds,
            edge_sets
        });

        return slider;
    }
}

impl Hold {
    pub fn from_split(split: Vec<&str>) -> HitObject {
        // 62:0:0:0:0:
        // end_time(62):hit_sample(0:0:0:0:)
        // the last entry in the split is two values ​​separated by a colon
        let mut last_entry = split[5].splitn(2, ":");
        let end_time = last_entry.next().unwrap_or_else(|| "0.0");
        let end_time = end_time.parse().unwrap_or_else(|_| 0.0);
        let mut hold = HitObject::from_split(&split);

        hold.hit_sample = last_entry.next().unwrap_or_else(|| "0:0:0:0:").to_string();
        hold.other = NoteType::Hold( Hold { end_time });

        return hold;
    }
}
