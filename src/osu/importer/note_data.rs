use crate::magic;
use crate::osu::note::{
    HitObject, NoteData, 
    Circle, Slider, Continuous, 
    HitSound
};

pub fn get_note_data(section: &Vec<String>) -> NoteData {
    let mut circles: Vec<HitObject<Circle>> = vec!();
    let mut sliders: Vec<HitObject<Slider>> = vec!();
    let mut continuous: Vec<HitObject<Continuous>> = vec!();

    for line in section {
        let split = line.split(",");
        let split = split.collect::<Vec<&str>>();

        let note_type = magic::convert::<u8>(&split[3], 1);

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

    NoteData {
        circles,
        sliders,
        continuous
    }
}

impl<T> HitObject<T> {
    pub fn from_split(split: &Vec<&str>, other: T) -> Self {
        let x =         magic::convert::<f32>(&split[0], 0.0);
        let y =         magic::convert::<f32>(&split[1], 0.0);
        let time =      magic::convert(&split[2], 0.0);
        let note_type = magic::convert::<u8>(&split[3], 0);
        let hit_sound:  HitSound = HitSound::new(magic::convert::<u8>(&split[4], 0));
        let hit_sample = String::new();
        
        Self {
            x, y, time,
            note_type,
            hit_sound,
            hit_sample,
            other
        }
    }
}

impl Circle {
    pub fn from_split(split: Vec<&str>)  -> HitObject<Self> {
        let mut circle = HitObject::<Self>::from_split(&split, Self {});
        circle.hit_sample = split[5].to_string();
        return circle
    }
}

impl Continuous {
    pub fn from_split(split: Vec<&str>) -> HitObject<Self> {
        let line_end_split = split[5].split(":");
        let mut line_end_split = line_end_split.collect::<Vec<&str>>();
        let end_time = magic::convert(line_end_split[0], 0.0);
        line_end_split.remove(0);
        let mut hit_sample = String::new();
        for element in line_end_split {
            hit_sample.push_str(element);
        }

        let mut continuous = HitObject::<Self>::from_split(
            &split, Self { end_time }
        );
        continuous.hit_sample = hit_sample;

        return continuous
    }
}

impl Slider {
    pub fn from_split(split: Vec<&str>) -> HitObject<Self> {
        let params =        split[5].to_string();
        let slides =        magic::convert(&split[6], 0);
        let length =        magic::convert(&split[7], 0.0);
        let edge_sounds: [HitSound; 2];
        let edge_sets: [String; 2];
        let hit_sample: String;

        match split.len() {
            8 => {
                edge_sounds = [HitSound::Normal, HitSound::Normal];
                edge_sets = ["0:0".to_string(), "0:0".to_string()];
                hit_sample = String::from("");
            }
            9 => {
                edge_sounds = [HitSound::Normal, HitSound::Normal];
                edge_sets = ["0:0".to_string(), "0:0".to_string()];
                hit_sample = split[8].to_string();
            }
            11 => {
                let help_me = split[8].split("|");
                let help_me = help_me.collect::<Vec<&str>>();
                edge_sounds = [
                    HitSound::new(magic::convert::<u8>(help_me[0], 0)),
                    HitSound::new(magic::convert::<u8>(help_me[1], 0)),
                ];
    
                let help_me = split[9].split("|");
                let help_me = help_me.collect::<Vec<&str>>();
                edge_sets = [
                    help_me[0].to_string(),
                    help_me[1].to_string()
                ];

                hit_sample = split[10].to_string();
            }
            _ => {
                edge_sounds = [HitSound::Normal, HitSound::Normal];
                edge_sets = ["0:0".to_string(), "0:0".to_string()];
                hit_sample = String::from("");
                println!("slider {} len is {}", split[2], split.len())
            }
        }

        let mut slider = HitObject::<Slider>::from_split(
            &split, Slider{
                params,
                slides,
                length,
                edge_sounds,
                edge_sets
            }
        );

        slider.hit_sample = hit_sample;
        return slider;
    }
}
