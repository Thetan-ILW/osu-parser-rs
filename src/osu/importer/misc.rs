use std::collections::{HashMap, BTreeMap};
use crate::osu::settings::{Events, Color};
use crate::osu::importer::helpers;

pub fn get_events(_section: &Vec<String>) -> Events {
    Events {}
}

pub fn get_colors(section: &Vec<String>) -> Vec<Color> {
    if section.len() == 0{
        return vec!()
    }

    let mut section_data: BTreeMap<String, String> = BTreeMap::new();
    helpers::get_key_value_btreemap(section, &mut section_data);

    let mut colors: Vec<Color> = vec!();

    for (_, value) in section_data {
        let split: Vec<&str> = value.split(",").collect();
        if split.len() != 3 {
            println!("Error: invalid color value");
            continue;
        }

        let r = helpers::convert::<u8>(split[0], 255);
        let g = helpers::convert::<u8>(split[1], 255);
        let b = helpers::convert::<u8>(split[2], 255);

        colors.push(Color(r, g, b));
    }

    return colors
}

