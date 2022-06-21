use crate::magic;

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
    pub effects: u8
}

impl TimePoint {
    pub fn from_split(split: Vec<&str>) -> Self {
        let time = magic::convert(&split[0], 0.0);
        let beat_length = magic::convert(&split[1], 0.0);
        let meter = magic::convert(&split[2], 0);

        let sample_set = magic::convert(&split[3], 0);
        let sample_set = match sample_set {
            0 => SampleSet::Default,
            1 => SampleSet::Normal,
            2 => SampleSet::Soft,
            3 => SampleSet::Drum,
            _ => SampleSet::Default
        };

        let sample_index = magic::convert(&split[4], 0);
        let volume = magic::convert(&split[5], 100.0);
        let uninherited = split[6].clone().parse::<bool>().unwrap_or(true); // idiot
        let effects = magic::convert::<u8>(&split[7], 0);
        Self {
            time,
            beat_length,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited,
            effects
        }
    }
}