use std::collections::BTreeMap;
use crate::osu::{Mode, SampleSet, OverlayPosition};
use crate::osu::settings::{General, Editor, Metadata, Difficulty};
use crate::osu::importer::key_value;

pub fn get_general(section: &Vec<String>) -> General {
    let mut section_data: BTreeMap<String, String> = BTreeMap::new();
    key_value::get_key_value(section, &mut section_data);

    let mut bool_data: BTreeMap<&str, bool> = BTreeMap::from([
        ("LetterboxInBreaks",       false), // Key name , Value || Default value
        ("UseSkinSprites",          false),
        ("EpilepsyWarning",         false),
        ("SpecialStyle",            false),
        ("WidescreenStoryboard",    false),
        ("SamplesMatchPlaybackRate",false)
    ]);
    
    let mut u32_data: BTreeMap<&str, u32> = BTreeMap::from([
        ("Countdown",       0),
        ("Mode",            0),
        ("CountdownOffset", 0)
    ]);

    let mut f64_data: BTreeMap<&str, f64> = BTreeMap::from([
        ("AudioLeadIn",     0.0),
        ("PreviewTime",     -1.0),
        ("StackLeniency",   0.7)
    ]);

    key_value::parse_and_set_bool(&mut bool_data, &section_data);
    key_value::parse_and_set(&mut u32_data, &section_data);
    key_value::parse_and_set(&mut f64_data, &section_data);

    let audio_filename          = key_value::get_safely(&section_data, "AudioFilename");
    let audio_lead_in           = f64_data["AudioLeadIn"];
    let preview_time            = f64_data["PreviewTime"];
    let countdown               = u32_data["Countdown"];

    let sample_set = SampleSet::from_string(
        key_value::get_safely(&section_data, "SampleSet")
    );

    let stack_leniency          = f64_data["StackLeniency"];
    let mode                    = Mode::new(u32_data["Mode"] as i8);
    let letter_box_in_breaks    = bool_data["LetterboxInBreaks"];
    let use_skin_sprites        = bool_data["UseSkinSprites"];

    let overlay_position = OverlayPosition::new(
        key_value::get_safely(&section_data, "OverlayPosition")
    );

    let skin_preference         = key_value::get_safely(&section_data, "SkinPreference");
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
    let mut section_data: BTreeMap<String, String> = BTreeMap::new();
    key_value::get_key_value(section, &mut section_data);

    let mut f32_data: BTreeMap<&str, f32> = BTreeMap::from([
        ("DistanceSpacing", 1.0),
        ("BeatDivisor",     16.0),
        ("GridSize",        32.0),
        ("TimelineZoom",    1.0)
    ]);

    key_value::parse_and_set(&mut f32_data, &section_data);

    let bookmarks_list = key_value::get_safely(&section_data, "Bookmarks");
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
    let mut section_data: BTreeMap<String, String> = BTreeMap::new();
    key_value::get_key_value(section, &mut section_data);

    let mut i32_data: BTreeMap<&str, i32> = BTreeMap::from([
        ("BeatmapID", 0),
        ("BeatmapSetID", 0)
    ]);

    key_value::parse_and_set(&mut i32_data, &section_data);

    let title           = key_value::get_safely(&section_data, "Title");
    let title_unicode   = key_value::get_safely(&section_data, "TitleUnicode");
    let artist          = key_value::get_safely(&section_data, "Artist");
    let artist_unicode  = key_value::get_safely(&section_data, "ArtistUnicode");
    let creator         = key_value::get_safely(&section_data, "Creator");
    let version         = key_value::get_safely(&section_data, "Version");
    let source          = key_value::get_safely(&section_data, "Source");  
    
    let tags = key_value::get_safely(&section_data, "Tags");
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
    let mut section_data: BTreeMap<String, String> = BTreeMap::new();
    key_value::get_key_value(section, &mut section_data);

    let mut f32_data: BTreeMap<&str, f32> = BTreeMap::from([
        ("HPDrainRate", 5.0),
        ("CircleSize", 5.0),
        ("OverallDifficulty", 5.0),
        ("ApproachRate", 5.0),
        ("SliderMultiplier", 1.0),
        ("SliderTickRate", 1.0)
    ]);

    key_value::parse_and_set(&mut f32_data, &section_data);

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