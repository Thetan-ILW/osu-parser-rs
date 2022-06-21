use crate::convert;

pub enum HitSound {
    Normal,
    Whistle,
    Finish,
    Clap
}

impl HitSound {
    pub fn new(value: u8) -> HitSound {
        match value{
            0 => HitSound::Normal,
            1 => HitSound::Whistle,
            2 => HitSound::Finish,
            3 => HitSound::Clap,
            _ => HitSound::Normal,
        }
    }
}

pub struct HitObjects {
    pub circles: Vec<HitObject<Circle>>,
    pub sliders: Vec<HitObject<Slider>>,
    pub continuous: Vec<HitObject<Continuous>>
}

pub struct HitObject<T> {
    pub x: f32,
    pub y: f32,
    pub time: f64,
    pub note_type: u8,
    pub hit_sound: HitSound,
    pub hit_sample: String,
    pub other: T
}

impl<T> HitObject<T> {
    pub fn from_split(split: &Vec<&str>, other: T) -> Self {
        let x =         convert::to_f32(&split[0]);
        let y =         convert::to_f32(&split[1]);
        let time =      convert::to_f64(&split[2]);
        let note_type = convert::to_u8(&split[3]);
        let hit_sound:  HitSound = HitSound::new(convert::to_u8(&split[4]));
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

pub struct Circle {}

pub struct Continuous {
    pub end_time: f64,
}

pub struct Slider {
    pub params: String,
    pub slides: u32,
    pub length: f64,
    pub edge_sounds: [HitSound; 2],
    pub edge_sets: [String; 2],
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
        let end_time = convert::to_f64(line_end_split[0]);
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
        let slides =        convert::to_u32(&split[6]);
        let length =        convert::to_f64(&split[7]);
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
                    HitSound::new(convert::to_u8(help_me[0])),
                    HitSound::new(convert::to_u8(help_me[1])),
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