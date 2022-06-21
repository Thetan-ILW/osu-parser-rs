use crate::convert;

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
        let time = convert::to_f64(split[0]);
        let beat_length = convert::to_f64(split[1]);
        let meter = convert::to_u32(split[2]);

        let sample_set = convert::to_u32(split[3]);
        let sample_set = match sample_set {
            0 => SampleSet::Default,
            1 => SampleSet::Normal,
            2 => SampleSet::Soft,
            3 => SampleSet::Drum,
            _ => SampleSet::Default
        };

        let sample_index = convert::to_u32(split[4]);
        let volume = convert::to_f32(split[5]);
        let uninherited = convert::to_bool(split[6]); 
        let effects = convert::to_u8(split[7]);
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