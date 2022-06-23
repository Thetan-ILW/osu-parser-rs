pub mod importer;
pub mod exporter;
pub mod settings;
pub mod timing;
pub mod note;

pub enum Mode {
    Osu,
    Taiko,
    Fruits,
    Mania,
    Unknown
}

impl Mode {
    pub fn new(value: i8) -> Self {
        match value {
            0 => Mode::Osu,
            1 => Mode::Taiko,
            2 => Mode::Fruits,
            3 => Mode::Mania,
            _ => Mode::Unknown
        }
    }
}

pub enum SampleSet {
    Default,
    Normal,
    Soft,
    Drum
}

impl SampleSet {
    pub fn new(value: i8) -> Self {
        match value {
            0 => SampleSet::Default,
            1 => SampleSet::Normal,
            2 => SampleSet::Soft,
            3 => SampleSet::Drum,
            _ => SampleSet::Default
        }
    }

}

pub struct Beatmap {
    pub settings: settings::Settings,
    pub timing_data: Vec<timing::TimePoint>,
    pub note_data: note::NoteData
}