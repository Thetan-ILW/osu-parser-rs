use crate::osu::{Import, Color};
use crate::osu::importer::key_value;
use crate::osu::sections::{Colors, Events};
use std::collections::BTreeMap;

impl Import for Events {
    fn parse(_section: &Vec<String>) -> Self {
        Events {}
    }
}

impl Import for Colors {
    fn parse(section: &Vec<String>) -> Self {
        let mut colors = Colors::default();

        if section.len() == 0 {
            return colors
        }
    
        let mut section_data: BTreeMap<String, String> = BTreeMap::new();
        key_value::get_key_value(section, &mut section_data);
    
        for (_, value) in section_data {
            let split: Vec<&str> = value.split(",").collect();
            if split.len() != 3 {
                println!("Error: invalid color value");
                continue;
            }
    
            let r = split[0].parse().unwrap_or_else(|_| 255);
            let g = split[1].parse().unwrap_or_else(|_| 255);
            let b = split[2].parse().unwrap_or_else(|_| 255);
    
            colors.data.push(Color(r, g, b));
        }
    
        return colors
    }
}
