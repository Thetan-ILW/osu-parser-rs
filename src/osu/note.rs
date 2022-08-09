pub struct HitObject {
    pub x: f32,
    pub y: f32,
    pub time: f64,
    pub note_type: u8,
    pub hit_sound: u8,
    pub hit_sample: String,
    pub additions: Additions,
}

#[derive(PartialEq)]
pub struct Circle;

#[derive(PartialEq)]
pub struct Spinner {
    pub end_time: f64,
}

#[derive(PartialEq)]
pub struct Hold {
    pub end_time: f64,
}

#[derive(PartialEq)]
pub struct Slider {
    pub params: String,
    pub slides: u32,
    pub length: f64,
    pub edge_sounds: [u8; 2],
    pub edge_sets: [String; 2],
}

#[derive(PartialEq)]
pub enum Additions {
    None,
    Circle(Circle),
    Slider(Slider),
    Spinner(Spinner),
    Hold(Hold)
}