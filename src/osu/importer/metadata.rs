use std::collections::HashMap;
use crate::magic;
use crate::osu::{Mode, SampleSet, OverlayPosition};
use crate::osu::settings::{
    General, Editor, Metadata, Difficulty, Events
};

pub fn get_general(section: &Vec<String>) -> General {
    let mut data: HashMap<String, String> = HashMap::new();

    for line in section {
        get_key_value(line, &mut data)
    }

    let mut bool_data: HashMap<&str, bool> = HashMap::from([
        ("LetterboxInBreaks",       false), // Key name , Value || Default value
        ("UseSkinSprites",          false),
        ("EpilepsyWarning",         false),
        ("SpecialStyle",            false),
        ("WidescreenStoryboard",    false),
        ("SamplesMatchPlaybackRate",false)
    ]);
    
    let mut u32_data: HashMap<&str, u32> = HashMap::from([
        ("Countdown",   0),
        ("Mode",        0),
        ("CountdownOffset", 0)
    ]);

    let mut f64_data: HashMap<&str, f64> = HashMap::from([
        ("AudioLeadIn", 0.0),
        ("PreviewTime",     -1.0),
        ("StackLeniency",   0.7)
    ]);

    for (name, value) in &mut bool_data {
        convert_bool(name, value, &data);
    }

    for (name, value) in &mut u32_data {
        convert::<u32>(name, value, &data);
    }
    
    for (name, value) in &mut f64_data {
        convert::<f64>(name, value, &data);
    }

    let audio_filename          = magic::get_value(&data, "AudioFilename", String::new());
    let audio_lead_in           = f64_data["AudioLeadIn"];
    let preview_time            = f64_data["PreviewTime"];
    let countdown               = u32_data["Countdown"];

    let sample_set              = SampleSet::from_string(
        magic::get_value(&data, 
            "SampleSet", 
            String::new()
        )
    );

    let stack_leniency          = f64_data["StackLeniency"];
    let mode                    = Mode::new(u32_data["Mode"] as i8);
    let letter_box_in_breaks    = bool_data["LetterboxInBreaks"];
    let use_skin_sprites        = bool_data["UseSkinSprites"];

    let overlay_position        = OverlayPosition::new(
        magic::get_value(&data, 
            "OverlayPosition", 
            String::new()
        )
    );

    let skin_preference         = magic::get_value(
        &data, 
        "SkinPreference", 
        String::new()
    );

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

    for line in section {
        get_key_value(line, &mut data)
    }

    let distance_spacing = magic::get_value::<f32>(&data, "DistanceSpacing", 1.0);
    let beat_divisor = magic::get_value::<f32>(&data, "BeatDivisor", 16.0);
    let grid_size = magic::get_value::<f32>(&data, "GridSize", 16.0);
    let timeline_zoom = magic::get_value::<f32>(&data, "TimelineZoom", 16.0);
    Editor {
        distance_spacing,
        beat_divisor,
        grid_size,
        timeline_zoom
    }
}

pub fn get_difficulty(section: &Vec<String>) -> Difficulty {
    let mut data: HashMap<String, String> = HashMap::new();
    for line in section {
        get_key_value(line, &mut data)
    }

    let hp_drain_rate = magic::get_value::<f32>(&data, "HPDrainRate", 5.0);
    let circle_size = magic::get_value::<f32>(&data, "CircleSize", 5.0);
    let overall_difficulty = magic::get_value::<f32>(&data, "OverallDifficulty", 5.0);
    let approach_rate = magic::get_value::<f32>(&data, "ApproachRate", 5.0);
    let slider_multiplier = magic::get_value::<f32>(&data, "SliderMultiplier", 1.0);
    let slider_tick_rate = magic::get_value::<f32>(&data, "SliderTickRate", 1.0);
    Difficulty {
        hp_drain_rate,
        circle_size,
        overall_difficulty,
        approach_rate,
        slider_multiplier,
        slider_tick_rate
    }
}

pub fn get_metadata(section: &Vec<String>) -> Metadata {
    let mut data: HashMap<String, String> = HashMap::new();

    for line in section {
        get_key_value(line, &mut data)
    }

    let title =          magic::get_value(&data, "Title", String::new());
    let title_unicode =  magic::get_value(&data, "TitleUnicode", String::new());
    let artist =         magic::get_value(&data, "Artist", String::new());
    let artist_unicode = magic::get_value(&data, "ArtistUnicode", String::new());
    let creator =        magic::get_value(&data, "Creator", String::new());
    let version =        magic::get_value(&data, "Version", String::new());

    Metadata{
        title,
        title_unicode,
        artist,
        artist_unicode,
        creator,
        version
    }
}

pub fn get_events(_section: &Vec<String>) -> Events
{
    Events {}
}

fn convert<T: std::str::FromStr>(name: &str, value: &mut T, data: &HashMap::<String, String>) {
    let string = data.get(name);
    let string = match string {
        Some(string) => string,
        None => {
            println!("{name} not found");
            return;
        }
    };

    let new_value = string.parse::<T>();
    match new_value {
        Ok(new_value) => *value = new_value,
        Err(_) => {
            println!("failed to read {name}");
            return;
        }
    };
}

fn convert_bool(name: &str, value: &mut bool, data: &HashMap::<String,String>) {
    let string = data.get(name);
    let string = match string {
        Some(string) => string,
        None => {
            println!("{name} not found");
            return
        }
    };

    match string {
        _ if string == "0" => *value = false,
        _ if string == "1" => *value = true,
        _ => {
            println!("failed to read {name}");
            return;
        }
    };
}

fn get_key_value(line: &String, data: &mut HashMap<String, String>) {
    let key_value = line.split(":");
    let key_value = key_value.collect::<Vec<&str>>();

    if key_value.len() == 2 {
        data.insert(
            key_value[0].trim().to_string(), 
            key_value[1].trim().to_string()
        );
    }
}