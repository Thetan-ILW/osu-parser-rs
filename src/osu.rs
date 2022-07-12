pub mod exporter;
pub mod importer;
pub mod note;
pub mod sections;

use crate::osu::sections::{
    General, Editor, Metadata, 
    Difficulty, Events, Colors, 
    TimingPoints, HitObjects
};

pub struct TimePoint {
    pub time: f64,
    pub beat_length: f64,
    pub meter: u32,
    pub sample_set: u8,
    pub sample_index: u32,
    pub volume: f32,
    pub uninherited: bool,
    pub effects: u8,
}

pub struct Event {
    pub e_type: String,
    pub start_time: f64,
    pub params: Vec<String>
}

#[derive(Clone)]
pub struct Color(pub u8, pub u8, pub u8);

pub struct Info { // I CANT FIND THE RIGHT NAME
    pub general: General,
    pub editor: Editor,
    pub metadata: Metadata,
    pub difficulty: Difficulty,
    pub events: Events,
    pub colors: Colors,
}

pub struct Beatmap {
    pub info: Info,
    pub timing_points: TimingPoints,
    pub hit_objects: HitObjects,
}