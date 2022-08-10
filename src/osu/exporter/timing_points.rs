use crate::osu;
use osu::exporter::Export;
use osu::sections::TimingPoints;

use std::fmt::Write;

impl Export for TimingPoints {
    fn as_string(&self) -> Result<String, std::fmt::Error> {
        let timing_points = &self.data;
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
}