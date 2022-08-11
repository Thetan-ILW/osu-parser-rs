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
                Additions::Circle(_)  => HitObjects::circle_as_string(item),
                Additions::Slider(additions) => HitObjects::slider_as_string(item, additions),
                Additions::Spinner(additions) => HitObjects::spinner_as_string(item, additions),
                Additions::Hold(additions) => HitObjects::hold_as_string(item, additions),
                Additions::None => panic!("Unknown object")
            };

            writeln!(&mut lines, "{line}")?;
        }

        return Ok(lines)
    }

    fn default_string() -> String {
        "[HitObjects]".to_string()
    }
}

impl HitObjects {
    fn circle_as_string(c: &HitObject) -> String{
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
    
    fn slider_as_string(s: &HitObject, additions: &Slider) -> String {
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
    
    fn spinner_as_string(s: &HitObject, additions: &Spinner) -> String {
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
    
    fn hold_as_string(h: &HitObject, additions: &Hold) -> String {
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
