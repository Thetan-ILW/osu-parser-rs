use crate::osu;
use osu::importer::Import;
use osu::sections::HitObjects;
use osu::note::{Circle, HitObject, HitSound, Hold, Slider, Spinner};

impl Import for HitObjects {
    fn parse(section: &Vec<String>) -> Self {
        let mut circles: Vec<HitObject<Circle>> = vec![];
        let mut sliders: Vec<HitObject<Slider>> = vec![];
        let mut spinners: Vec<HitObject<Spinner>> = vec![];
        let mut holds: Vec<HitObject<Hold>> = vec![];
    
        for line in section {
            let split = line.split(",");
            let split = split.collect::<Vec<&str>>();
    
            let note_type: u8 = split[3].parse().unwrap_or(0);
            // https://github.com/nojhamster/osu-parser/blob/master/index.js
            // stole the code from there ^^^, because i'm DUMB 🤪 to think for myself
            if (1 & note_type) == 1 {
                let circle = Circle::from_split(split);
                circles.push(circle);
            }
            else if (2 & note_type) == 2 {
                let slider = Slider::from_split(split);
                sliders.push(slider);
            }
            else if (8 & note_type) == 8 {
                let spinner = Spinner::from_split(split);
                spinners.push(spinner);
            }
            else if (128 & note_type) == 128 {
                let hold = Hold::from_split(split);
                holds.push(hold);
            }
            else {
                println!("😡 UNKNOWN OBJECT {note_type}");
            }
        }
    
        HitObjects {
            circles,
            sliders,
            spinners,
            holds,
        }
    }
}

impl<T> HitObject<T> {
    pub fn from_split(split: &Vec<&str>, other: T) -> Self {
        let x = split[0].parse().unwrap_or_else(|_| 0.0);
        let y = split[1].parse().unwrap_or_else(|_| 0.0);
        let time = split[2].parse().unwrap_or_else(|_| 0.0);
        let note_type = split[3].parse().unwrap_or_else(|_| 0);

        let hit_sound: HitSound = HitSound::new(split[4].parse().unwrap_or(0));

        let hit_sample = String::new();

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
    pub fn from_split(split: Vec<&str>) -> HitObject<Self> {
        let mut circle = HitObject::<Self>::from_split(&split, Self {});
        circle.hit_sample = split[5].to_string();
        return circle;
    }
}

impl Spinner {
    pub fn from_split(split: Vec<&str>) -> HitObject<Self> {
        let end_time = split[5].parse().unwrap_or_else(|_| 0.0);
        let mut spinner = HitObject::<Self>::from_split(&split, Self { end_time });
        spinner.hit_sample = split[5].to_string();

        return spinner;
    }
}

impl Slider {
    pub fn from_split(split: Vec<&str>) -> HitObject<Self> {
        let params = split[5].to_string();
        let slides = split[6].parse().unwrap_or_else(|_| 0);
        let length = split[7].parse().unwrap_or_else(|_| 0.0);
        let mut edge_sounds: [HitSound; 2] = [HitSound::Normal, HitSound::Normal];
        let mut edge_sets: [String; 2] = ["0:0".to_string(), "0:0".to_string()];
        let hit_sample: String;

        match split.len() {
            8 => {
                hit_sample = String::from("");
            }
            9 => {
                hit_sample = split[8].to_string();
            }
            11 => {
                let help_me = split[8].split("|");
                let help_me = help_me.collect::<Vec<&str>>();
                edge_sounds = [
                    HitSound::new(help_me[0].parse().unwrap_or_else(|_| 0)),
                    HitSound::new(help_me[1].parse().unwrap_or_else(|_| 0)),
                ];

                let help_me = split[9].split("|");
                let help_me = help_me.collect::<Vec<&str>>();
                edge_sets = [help_me[0].to_string(), help_me[1].to_string()];

                hit_sample = split[10].to_string();
            }
            _ => {
                hit_sample = String::from("");
                println!("slider {} split.len is {}", split[2], split.len())
            }
        }

        let mut slider = HitObject::<Slider>::from_split(
            &split,
            Slider {
                params,
                slides,
                length,
                edge_sounds,
                edge_sets,
            },
        );

        slider.hit_sample = hit_sample;
        return slider;
    }
}

impl Hold {
    pub fn from_split(split: Vec<&str>) -> HitObject<Self> {
        let mut last = split[5].splitn(2, ":");
        let end_time = last.next().unwrap_or_else(|| "0.0");
        let end_time = end_time.parse().unwrap_or_else(|_| 0.0);
        let mut hold = HitObject::<Self>::from_split(&split, Self { end_time });
        hold.hit_sample = last.next().unwrap_or_else(|| ":0:0:0:0:").to_string();

        return hold;
    }
}
