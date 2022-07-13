use crate::osu;
use osu::sections::{Events, Colors};

use std::fmt::Write;
use std::fmt::Error;

pub fn get_events(e: &Events) -> Result<String, Error> {
    let events = &e.data;
    let mut lines = String::new();

    writeln!(&mut lines, "[Events]")?;

    for event in events {
        let e_type = event.e_type.clone();
        let start_time = event.start_time;
        let params = event.params.join(",");
        writeln!(&mut lines, "{},{},{}", e_type, start_time, params)?;
    }

    return Ok(lines)
}

pub fn get_colors(colors: &Colors) -> Result<String, Error> {
    let colors = &colors.data;
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