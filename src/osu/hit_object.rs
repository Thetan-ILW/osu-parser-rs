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

impl Continuous {
    pub fn from_split(split: Vec<&str>) -> (Self, String) {
        let line_end_split = split[5].split(":");
        let mut line_end_split = line_end_split.collect::<Vec<&str>>();
        let end_time = convert::to_f64(line_end_split[0]);
        line_end_split.remove(0);
        let mut hit_sample = String::new();
        for element in split {
            hit_sample.push_str(element);
        }

        (Self { end_time }, hit_sample)
    }
}

impl Slider {
    pub fn from_split(split: Vec<&str>) -> (Self, String) {
        let params =        split[5].to_string();
        let slides =        convert::to_u32(&split[6]);
        let length =        convert::to_f64(&split[7]);
        let edge_sounds: [HitSound; 2];
        let edge_sets: [String; 2];
        let hit_sample: String;

        if split.len() == 11 {
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
        else {
            edge_sounds = [HitSound::Normal, HitSound::Normal];
            edge_sets = ["0:0".to_string(), "0:0".to_string()];
            hit_sample = split[7].to_string(); // TESTS
        }

        (Self { 
            params,
            slides,
            length,
            edge_sounds,
            edge_sets
        }, hit_sample)

    }
}