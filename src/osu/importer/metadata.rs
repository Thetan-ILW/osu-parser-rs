use crate::osu::importer::key_value;
use crate::osu::sections::{Difficulty, Editor, General, Metadata};
use crate::osu::{Mode, OverlayPosition, SampleSet, Import};
use std::collections::BTreeMap;

impl Import for General {
    fn parse(strings: &Vec<String>) -> Self{
        let mut section_data: BTreeMap<String, String> = BTreeMap::new();
        key_value::get_key_value(strings, &mut section_data);
    
        let mut bool_data: BTreeMap<&str, bool> = BTreeMap::from([
            ("LetterboxInBreaks", false), // Key; Value || Default value
            ("UseSkinSprites", false),
            ("EpilepsyWarning", false),
            ("SpecialStyle", false),
            ("WidescreenStoryboard", false),
            ("SamplesMatchPlaybackRate", false),
        ]);
    
        let mut u32_data: BTreeMap<&str, u32> =
            BTreeMap::from([("Countdown", 0), ("Mode", 0), ("CountdownOffset", 0)]);
    
        let mut f64_data: BTreeMap<&str, f64> = BTreeMap::from([
            ("AudioLeadIn", 0.0),
            ("PreviewTime", -1.0),
            ("StackLeniency", 0.7),
        ]);
    
        key_value::parse_and_set_bool(&mut bool_data, &section_data);
        key_value::parse_and_set(&mut u32_data, &section_data);
        key_value::parse_and_set(&mut f64_data, &section_data);
    
        let mut g = General::default();
    
        g.audio_filename = key_value::get_safely(&section_data, "AudioFilename");
        g.audio_lead_in = f64_data["AudioLeadIn"];
        g.preview_time = f64_data["PreviewTime"];
        g.countdown = u32_data["Countdown"];
    
        g.sample_set = SampleSet::from_string(
            key_value::get_safely(&section_data, "SampleSet")
        );
    
        g.stack_leniency = f64_data["StackLeniency"];
        g.mode = Mode::new(u32_data["Mode"] as i8);
        g.letter_box_in_breaks = bool_data["LetterboxInBreaks"];
        g.use_skin_sprites = bool_data["UseSkinSprites"];
    
        g.overlay_position =
            OverlayPosition::new(key_value::get_safely(&section_data, "OverlayPosition"));
    
        g.skin_preference = key_value::get_safely(&section_data, "SkinPreference");
        g.epilepsy_warning = bool_data["EpilepsyWarning"];
        g.countdown_offset = u32_data["CountdownOffset"];
        g.special_style = bool_data["SpecialStyle"];
        g.widescreen_storyboard = bool_data["WidescreenStoryboard"];
        g.samples_match_playback_rate = bool_data["SamplesMatchPlaybackRate"];
    
        return g;
    }
}

impl Import for Editor {
    fn parse(section: &Vec<String>) -> Self{
        let mut section_data: BTreeMap<String, String> = BTreeMap::new();
        key_value::get_key_value(section, &mut section_data);
    
        let mut f32_data: BTreeMap<&str, f32> = BTreeMap::from([
            ("DistanceSpacing", 1.0),
            ("BeatDivisor", 16.0),
            ("GridSize", 32.0),
            ("TimelineZoom", 1.0),
        ]);
    
        key_value::parse_and_set(&mut f32_data, &section_data);
    
        let mut e = Editor::default();
    
        let bookmarks_list = key_value::get_safely(&section_data, "Bookmarks");
        let bookmarks_list: Vec<&str> = bookmarks_list.split(",").collect();
        if bookmarks_list.len() > 1 {
            for item in bookmarks_list {
                e.bookmarks
                    .push(item.parse::<f64>().unwrap_or_else(|_| 0.0));
            }
        }
    
        e.distance_spacing = f32_data["DistanceSpacing"];
        e.beat_divisor = f32_data["BeatDivisor"];
        e.grid_size = f32_data["GridSize"];
        e.timeline_zoom = f32_data["TimelineZoom"];
    
        return e;
    }
}

impl Import for Metadata {
    fn parse(section: &Vec<String>) -> Self {
        let mut section_data: BTreeMap<String, String> = BTreeMap::new();
        key_value::get_key_value(section, &mut section_data);
    
        let mut i32_data: BTreeMap<&str, i32> = BTreeMap::from([("BeatmapID", 0), ("BeatmapSetID", 0)]);
    
        key_value::parse_and_set(&mut i32_data, &section_data);
    
        let mut m = Metadata::default();
    
        m.title = key_value::get_safely(&section_data, "Title");
        m.title_unicode = key_value::get_safely(&section_data, "TitleUnicode");
        m.artist = key_value::get_safely(&section_data, "Artist");
        m.artist_unicode = key_value::get_safely(&section_data, "ArtistUnicode");
        m.creator = key_value::get_safely(&section_data, "Creator");
        m.version = key_value::get_safely(&section_data, "Version");
        m.source = key_value::get_safely(&section_data, "Source");
    
        let tags = key_value::get_safely(&section_data, "Tags");
        let tags: Vec<&str> = tags.split(" ").collect();
        m.tags = tags.iter().map(|&s| s.into()).collect();
    
        m.beatmap_id = i32_data["BeatmapID"];
        m.beatmap_set_id = i32_data["BeatmapSetID"];
    
        return m;
    }
}

impl Import for Difficulty {
    fn parse(section: &Vec<String>) -> Self {
        let mut section_data: BTreeMap<String, String> = BTreeMap::new();
        key_value::get_key_value(section, &mut section_data);
    
        let mut f32_data: BTreeMap<&str, f32> = BTreeMap::from([
            ("HPDrainRate", 5.0),
            ("CircleSize", 5.0),
            ("OverallDifficulty", 5.0),
            ("ApproachRate", 5.0),
            ("SliderMultiplier", 1.0),
            ("SliderTickRate", 1.0),
        ]);
    
        key_value::parse_and_set(&mut f32_data, &section_data);
    
        let mut d = Difficulty::default();
    
        d.hp_drain_rate = f32_data["HPDrainRate"];
        d.circle_size = f32_data["CircleSize"];
        d.overall_difficulty = f32_data["OverallDifficulty"];
        d.approach_rate = f32_data["ApproachRate"];
        d.slider_multiplier = f32_data["SliderMultiplier"];
        d.slider_tick_rate = f32_data["SliderTickRate"];
    
        return d;
    }
}
