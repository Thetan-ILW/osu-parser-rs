pub enum HitSound {
    Normal,
    Whistle,
    Finish,
    Clap
}

pub enum SampleSet {
    Default,
    Normal,
    Soft,
    Drum
}

pub struct TimePoint {
    pub time: f64,
    pub beat_length: f64,
    pub meter: u32,
    pub sample_set: SampleSet,
    pub sample_index: u32,
    pub volume: f32,
    pub uninherited: bool,
    pub effects: u32 // idk
}

pub struct HitObject {   // https://osu.ppy.sh/wiki/en/Client/File_formats/Osu_%28file_format%29
    pub x: f32,
    pub y: f32,
    pub time: f64,
    pub note_type: u8,
    pub hit_sound: HitSound,
    pub end_time: f64,
    pub hit_sample: String
}
