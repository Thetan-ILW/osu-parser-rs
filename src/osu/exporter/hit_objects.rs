use crate::osu;
use osu::exporter::Export;
use osu::note::{Additions, HitObject, Slider, Spinner, Hold};
use osu::sections::HitObjects;

use std::fmt::Write;

impl Export for HitObjects {
    fn as_string(&self) -> Result<String, std::fmt::Error> {
        let mut lines = String::new();

        writeln!(&mut lines, "[HitObjects]")?;

        for item in &self.data {
            let line = match &item.additions {
                Additions::Circle(_)  => HitObjects::get_circle_line(item),
                Additions::Slider(additions) => HitObjects::get_slider_line(item, additions),
                Additions::Spinner(additions) => HitObjects::get_spinner_line(item, additions),
                Additions::Hold(additions) => HitObjects::get_hold_line(item, additions),
                Additions::None => panic!("Unknown object")
            };

            writeln!(&mut lines, "{line}")?;
        }

        return Ok(lines)
    }
}

impl HitObjects {
    fn get_circle_line(c: &HitObject) -> String{
        let line = format!(
            "{},{},{},{},{},{}",
            c.x,
            c.y,
            c.time,
            c.note_type,
            c.hit_sound,
            c.hit_sample
        );
    
        return line
    }
    
    fn get_slider_line(s: &HitObject, additions: &Slider) -> String {
        let edge_sounds = format!(
            "{}|{}",
            additions.edge_sounds[0],
            additions.edge_sounds[1]
        );
    
        let edge_sets = format!(
            "{}|{}",
            additions.edge_sets[0],
            additions.edge_sets[1]
        );
    
        let line = format!(
            "{},{},{},{},{},{},{},{},{},{},{}",
            s.x,
            s.y,
            s.time,
            s.note_type,
            s.hit_sound,
            additions.params,
            additions.slides,
            additions.length,
            edge_sounds,
            edge_sets,
            s.hit_sample
        );
    
        return line
    }
    
    fn get_spinner_line(s: &HitObject, additions: &Spinner) -> String {
        let line = format!(
            "{},{},{},{},{},{},{}",
            s.x,
            s.y,
            s.time,
            s.note_type,
            s.hit_sound,
            additions.end_time,
            s.hit_sample
        );
    
        return line
    }
    
    fn get_hold_line(h: &HitObject, additions: &Hold) -> String {
        let line = format!(
            "{},{},{},{},{},{}:{}",
            h.x,
            h.y,
            h.time,
            h.note_type,
            h.hit_sound,
            additions.end_time,
            h.hit_sample
        );
    
        return line
    }
}
