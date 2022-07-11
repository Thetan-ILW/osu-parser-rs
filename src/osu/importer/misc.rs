use std::collections::BTreeMap;

use crate::osu;
use osu::Color;
use osu::event::Event;
use osu::importer::Import;
use osu::importer::key_value;
use osu::sections::{Colors, Events};

impl Import for Events {
    fn parse(section: &Vec<String>) -> Self {
        let mut events: Vec<Event> = vec![];

        for line in section {
            if line.len() == 0 {
                continue;
            }

            if let Some(c) = line.chars().next() {
                if c == '/' {
                    continue;
                }
            }
            
            let split: Vec<&str> = line.splitn(3, ',').collect();

            if split.len() != 3 {
                continue;
            }

            let e_type = split[0].to_string();
            let start_time = split[1].parse::<f64>().unwrap_or_default();
            let params: Vec<&str> = split[2].split(",").collect();
            let params: Vec<String> = params.iter().map(|&s|s.into()).collect();

            events.push(Event {
                e_type,
                start_time,
                params
            })
        };

        Events {
            data: events
        }
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
                println!("😡 Error: invalid color value");
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
