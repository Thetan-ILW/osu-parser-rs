use std::collections::HashMap;
use crate::osu::{Mode, SampleSet, OverlayPosition};
use crate::osu::settings::{
    General, Editor, Metadata, Difficulty, Events
};

pub fn get_general(section: &Vec<String>) -> General {
    let mut data: HashMap<String, String> = HashMap::new();
    get_key_value(section, &mut data);

    let mut bool_data: HashMap<&str, bool> = HashMap::from([
        ("LetterboxInBreaks",       false), // Key name , Value || Default value
        ("UseSkinSprites",          false),
        ("EpilepsyWarning",         false),
        ("SpecialStyle",            false),
        ("WidescreenStoryboard",    false),
        ("SamplesMatchPlaybackRate",false)
    ]);
    
    let mut u32_data: HashMap<&str, u32> = HashMap::from([
        ("Countdown",       0),
        ("Mode",            0),
        ("CountdownOffset", 0)
    ]);

    let mut f64_data: HashMap<&str, f64> = HashMap::from([
        ("AudioLeadIn",     0.0),
        ("PreviewTime",     -1.0),
        ("StackLeniency",   0.7)
    ]);

    for (name, value) in &mut bool_data {
        parse_and_set_bool(name, value, &data);
    }

    for (name, value) in &mut u32_data {
        parse_and_set::<u32>(name, value, &data);
    }
    
    for (name, value) in &mut f64_data {
        parse_and_set::<f64>(name, value, &data);
    }

    let audio_filename          = get_safely(&data, "AudioFilename");
    let audio_lead_in           = f64_data["AudioLeadIn"];
    let preview_time            = f64_data["PreviewTime"];
    let countdown               = u32_data["Countdown"];

    let sample_set = SampleSet::from_string(
        get_safely(&data, "SampleSet")
    );

    let stack_leniency          = f64_data["StackLeniency"];
    let mode                    = Mode::new(u32_data["Mode"] as i8);
    let letter_box_in_breaks    = bool_data["LetterboxInBreaks"];
    let use_skin_sprites        = bool_data["UseSkinSprites"];

    let overlay_position = OverlayPosition::new(
        get_safely(&data, "OverlayPosition")
    );

    let skin_preference         = get_safely(&data, "SkinPreference");
    let epilepsy_warning        = bool_data["EpilepsyWarning"];
    let countdown_offset        = u32_data["CountdownOffset"];
    let special_style           = bool_data["SpecialStyle"];
    let widescreen_storyboard   = bool_data["WidescreenStoryboard"];
    let samples_match_playback_rate = bool_data["SamplesMatchPlaybackRate"];

    General {
        audio_filename,
        audio_lead_in,
        preview_time,
        countdown,
        sample_set,
        stack_leniency,
        mode,
        letter_box_in_breaks,
        use_skin_sprites,
        overlay_position,
        skin_preference,
        epilepsy_warning,
        countdown_offset,
        special_style,
        widescreen_storyboard,
        samples_match_playback_rate
    }
}

pub fn get_editor(section: &Vec<String>) -> Editor {
    let mut data: HashMap<String, String> = HashMap::new();
    get_key_value(section, &mut data);

    let mut f32_data: HashMap<&str, f32> = HashMap::from([
        ("DistanceSpacing", 1.0),
        ("BeatDivisor",     16.0),
        ("GridSize",        32.0),
        ("TimelineZoom",    1.0)
    ]);

    for (name, value) in &mut f32_data {
        parse_and_set::<f32>(name, value, &data);
    }

    let bookmarks_list = get_safely(&data, "Bookmarks");
    let bookmarks_list: Vec<&str> = bookmarks_list.split(",").collect();
    let mut bookmarks: Vec<f64> = vec!();
    if bookmarks_list.len() > 1 {
        for item in bookmarks_list {
            bookmarks.push(
                item.parse::<f64>().unwrap_or_else(|_| 0.0)
            );
        }
    }

    let distance_spacing = f32_data["DistanceSpacing"];
    let beat_divisor = f32_data["BeatDivisor"];
    let grid_size = f32_data["GridSize"];
    let timeline_zoom = f32_data["TimelineZoom"];

    Editor {
        bookmarks,
        distance_spacing,
        beat_divisor,
        grid_size,
        timeline_zoom
    }
}

pub fn get_metadata(section: &Vec<String>) -> Metadata {
    let mut data: HashMap<String, String> = HashMap::new();
    get_key_value(section, &mut data);

    let mut i32_data: HashMap<&str, i32> = HashMap::from([
        ("BeatmapID", 0),
        ("BeatmapSetID", 0)
    ]);

    for (name, value) in &mut i32_data {
        parse_and_set::<i32>(name, value, &data);
    }

    let title           = get_safely(&data, "Title");
    let title_unicode   = get_safely(&data, "TitleUnicode");
    let artist          = get_safely(&data, "Artist");
    let artist_unicode  = get_safely(&data, "ArtistUnicode");
    let creator         = get_safely(&data, "Creator");
    let version         = get_safely(&data, "Version");
    let source          = get_safely(&data, "Source");  
    
    let tags = get_safely(&data, "Tags");
    let tags: Vec<&str> = tags.split(" ").collect();
    let tags: Vec<String> = tags.iter()
        .map(|&s|s.into())
        .collect();

    let beatmap_id      = i32_data["BeatmapID"];
    let beatmap_set_id  = i32_data["BeatmapSetID"];

    Metadata{
        title,
        title_unicode,
        artist,
        artist_unicode,
        creator,
        version,
        source,
        tags,
        beatmap_id,
        beatmap_set_id
    }
}

pub fn get_difficulty(section: &Vec<String>) -> Difficulty {
    let mut data: HashMap<String, String> = HashMap::new();
    get_key_value(section, &mut data);

    let mut f32_data: HashMap<&str, f32> = HashMap::from([
        ("HPDrainRate", 5.0),
        ("CircleSize", 5.0),
        ("OverallDifficulty", 5.0),
        ("ApproachRate", 5.0),
        ("SliderMultiplier", 1.0),
        ("SliderTickRate", 1.0)
    ]);

    for (name, value) in &mut f32_data {
        parse_and_set::<f32>(name, value, &data);
    }

    let hp_drain_rate =         f32_data["HPDrainRate"];
    let circle_size =           f32_data["CircleSize"];
    let overall_difficulty =    f32_data["OverallDifficulty"];
    let approach_rate =         f32_data["ApproachRate"];
    let slider_multiplier =     f32_data["SliderMultiplier"];
    let slider_tick_rate =      f32_data["SliderTickRate"];

    Difficulty {
        hp_drain_rate,
        circle_size,
        overall_difficulty,
        approach_rate,
        slider_multiplier,
        slider_tick_rate
    }
}

pub fn get_events(_section: &Vec<String>) -> Events
{
    Events {}
}

// works with numbers only
fn parse_and_set<T: std::str::FromStr>(name: &str, value: &mut T, data: &HashMap::<String, String>) {
    let s = get_safely(data, name);
    let new_value = s.parse::<T>();
    
    match new_value {
        Ok(new_value) => *value = new_value,
        Err(_) => {
            println!("failed to read {name}");
            return;
        }
    };
}

// idk
fn parse_and_set_bool(name: &str, value: &mut bool, data: &HashMap::<String,String>) {
    let s = get_safely(data, name);

    match s {
        _ if s == "0" => *value = false,
        _ if s == "1" => *value = true,
        _ => {
            println!("failed to read {name}");
            return;
        }
    };
}

// Get the value from hashmap, if there is no or an error then return default value
// use only for strings
pub fn get_safely(data: &HashMap<String, String>, name: &str) -> String {
    match data.contains_key(name) {
        true => return data[name].clone(),
        false => {
            println!("key {name} does not exist");
            return String::new()
        }
    }
}

// Split [KEY:VALUE] and insert it to hashmap
fn get_key_value(section: &Vec<String>, data: &mut HashMap<String, String>) {
    for line in section {
        let key_value = line.split(":");
        let key_value = key_value.collect::<Vec<&str>>();
    
        if key_value.len() == 2 {
            data.insert(
                key_value[0].trim().to_string(), 
                key_value[1].trim().to_string()
            );
        }
    }
}