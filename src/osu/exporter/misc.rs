use crate::osu;
use osu::sections::Colors;

use std::fmt::Write;

pub fn get_colors(colors: &Colors) -> String {
    let colors = &colors.data;
    let mut lines = String::new();

    if colors.len() == 0 {
        return lines
    }

    if let Err(e) = writeln!(&mut lines, "[Colours]") {
        println!("{e}");
        return lines
    }

    let mut i = 1;
    for color in colors {
        if let Err(e) = writeln!(
            &mut lines, 
            "Combo{i} : {}, {}, {}", color.0, color.1, color.2
        ) {
            println!("{e}");
        }


        i += 1
    }

    return lines
}