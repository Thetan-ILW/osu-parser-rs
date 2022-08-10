use crate::osu;
use osu::exporter::Export;
use osu::sections::{Events, Colors};

use std::fmt::Write;

impl Export for Events {
    fn as_string(&self) -> Result<String, std::fmt::Error> {
        let events = &self.data;
        let mut lines = String::new();

        writeln!(&mut lines, "[Events]")?;

        for event in events {
            let e_type = event.e_type.clone();
            let start_time = event.start_time;
            let mut params = event.params.clone();

            if e_type == "0" {
                params[0] = format!(r#""{}""#, params[0]);
            }

            let params = params.join(",");
            writeln!(&mut lines, "{},{},{}", e_type, start_time, params)?;
        }

        return Ok(lines)
    }
}

impl Export for Colors {
    fn as_string(&self) -> Result<String, std::fmt::Error> {
        let colors = &self.data;
        let mut lines = String::new();

        if colors.len() == 0 {
            return Ok(lines)
        }

        writeln!(&mut lines, "[Colours]")?;

        let mut i = 1;
        for color in colors {
            writeln!(
                &mut lines, 
                "Combo{i} : {},{},{}", color.0, color.1, color.2
            )?;

            i += 1
        }

        return Ok(lines)
    }
}