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
        let line = format!(
            "{},{},{},{},{},{},{},{}",
            t.time,
            t.beat_length,
            t.meter,
            t.sample_set,
            t.sample_index,
            t.volume,
            t.uninherited as i32,
            t.effects
        );
        writeln!(&mut lines, "{line}").unwrap();
    }

    return lines
}