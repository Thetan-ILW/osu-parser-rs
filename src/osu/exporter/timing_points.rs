use crate::osu;
use osu::sections::TimingPoints;

use std::fmt::Write;
use std::fmt::Error;

pub fn get(timing_points: &TimingPoints) -> Result<String, Error> {
    let timing_points = &timing_points.data;
    let mut lines = String::new();

    writeln!(&mut lines, "[TimingPoints]")?;
    
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
        writeln!(&mut lines, "{line}")?;
    }

    return Ok(lines)
}