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

pub struct NoteData {
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