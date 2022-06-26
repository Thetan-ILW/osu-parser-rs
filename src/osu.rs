pub mod exporter;
pub mod importer;
pub mod note;
pub mod sections;
pub mod timing;

use crate::osu::sections::{
    General, Editor, Metadata, 
    Difficulty, Events, Colors, 
    TimingPoints, HitObjects
};

pub enum Mode {
    Osu,
    Taiko,
    Fruits,
    Mania,
    Unknown,
}

impl Mode {
    pub fn new(value: i8) -> Self {
        match value {
            0 => Mode::Osu,
            1 => Mode::Taiko,
            2 => Mode::Fruits,
            3 => Mode::Mania,
            _ => Mode::Unknown,
        }
    }
}

pub enum SampleSet {
    Default,
    Normal,
    Soft,
    Drum,
}

impl SampleSet {
    pub fn new(value: i8) -> Self {
        match value {
            0 => SampleSet::Default,
            1 => SampleSet::Normal,
            2 => SampleSet::Soft,
            3 => SampleSet::Drum,
            _ => SampleSet::Default,
        }
    }

    pub fn from_string(value: String) -> Self {
        let value: i8 = match value {
            _ if value == "Default" => 0,
            _ if value == "Normal" => 1,
            _ if value == "Soft" => 2,
            _ if value == "Drum" => 3,
            _ => 0,
        };

        Self::new(value)
    }
}

pub enum OverlayPosition {
    NoChange,
    Below,
    Above,
}

impl OverlayPosition {
    pub fn new(string: String) -> Self {
        match string {
            _ if string == "NoChange" => Self::NoChange,
            _ if string == "Below" => Self::Below,
            _ if string == "Above" => Self::Above,
            _ => Self::NoChange,
        }
    }
}

#[derive(Clone)]
pub struct Color(pub u8, pub u8, pub u8);

pub struct Settings { // I CANT FIND THE RIGHT NAME
    pub general: General,
    pub editor: Editor,
    pub metadata: Metadata,
    pub difficulty: Difficulty,
    pub events: Events,
    pub colors: Colors,
}

pub struct Beatmap {
    pub settings: Settings,
    pub timing_points: TimingPoints,
    pub hit_objects: HitObjects,
}

pub trait Import { // move to importer.rs pls
    fn parse(strings: &Vec<String>) -> Self; // rename this
}
