use crate::osu::SampleSet;

pub struct TimePoint {
    pub time: f64,
    pub beat_length: f64,
    pub meter: u32,
    pub sample_set: SampleSet,
    pub sample_index: u32,
    pub volume: f32,
    pub uninherited: bool,
    pub effects: u8,
}
