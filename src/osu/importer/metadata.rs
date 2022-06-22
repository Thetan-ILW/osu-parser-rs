use std::collections::HashMap;
use crate::magic;
use crate::osu::{Mode, SampleSet};
use crate::osu::settings::{
    General, Editor, Metadata, Difficulty, Events
};

pub fn get_general(section: &Vec<String>) -> General {
    let mut data: HashMap<String, String> = HashMap::new();

    for line in section {
        get_key_value(line, &mut data)
    }

    let audio_filename = magic::get_value(&data, "AudioFilename", String::new());
    let preview_time = magic::get_value::<f64>(&data, "PreviewTime", 0.0);
    let countdown = magic::get_value::<i8>(&data, "Countdown", 0);
    let countdown = magic::idiot(countdown, false);
    let sample_set = {
        let value = &data["SampleSet"];
        let value = match data["SampleSet"] {
            _ if value == "Default" => 0,
            _ if value == "Normal" => 1,
            _ if value == "Soft" => 2,
            _ if value == "Drum" => 3,
            _ => 1
        };
        SampleSet::new(value)
    };
    let stack_leniency = magic::get_value::<f32>(&data, "StackLeniency", 0.0);
    let mode = {
        let value = magic::get_value::<i8>(&data, "Mode", 4);
        Mode::new(value)
    };
    let letter_box_in_breaks = magic::get_value::<i8>(&data, "LetterboxInBreaks", 0);
    let widescreen_storyboard = magic::get_value::<i8>(&data, "WidescreenStoryboard", 0);
    let samples_match_playback = magic::get_value::<i8>(&data, "SamplesMatchPlaybackRate", 0);
    
    let letter_box_in_breaks = magic::idiot(letter_box_in_breaks, false);
    let widescreen_storyboard = magic::idiot(widescreen_storyboard, false);
    let samples_match_playback = magic::idiot(samples_match_playback, false);

    General{
        audio_filename,
        preview_time,
        countdown,
        sample_set,
        stack_leniency,
        mode,
        letter_box_in_breaks,
        widescreen_storyboard,
        samples_match_playback
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