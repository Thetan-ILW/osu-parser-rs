use crate::osu;
use osu::sections::TimingPoints;

use std::fmt::Write;

pub fn get(timing_points: &TimingPoints) -> String {
    let timing_points = &timing_points.data;
    let mut lines = String::new();

    if let Err(e) = writeln!(&mut lines, "[TimingPoints]") {
        println!("{e}");
        return lines
    }
    
    for t in timing_points {
        let time = t.time;
        let beat_length = t.beat_length;
        let meter = t.meter;
        let sample_set = t.sample_set;
        let sample_index = t.sample_index;
        let volume = t.volume;
        let uninherited = t.uninherited as i32;
        let effects = t.effects;

        let line = format!(
            "{time},{beat_length},{meter},{sample_set},{sample_index},{volume},{uninherited},{effects}"
        );
        writeln!(&mut lines, "{line}").unwrap();
    }

    return lines
}