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