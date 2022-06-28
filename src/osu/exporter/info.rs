use crate::osu;
use osu::sections::{General, Editor, Metadata, Difficulty};

use std::fmt::Write;

pub fn get_general(g: &General) -> String {
    let mut lines = String::new();
    if let Err(e) = writeln!(&mut lines, "[General]") {
        println!("{e}");
        return lines
    }

    if g.audio_filename.len() != 0 {
        writeln!(&mut lines, "AudioFilename: {}", g.audio_filename).unwrap();
    }
    
    writeln!(&mut lines, "AudioLeadIn: {}", g.audio_lead_in).unwrap();
    writeln!(&mut lines, "PreviewTime: {}", g.preview_time).unwrap();
    writeln!(&mut lines, "Countdown: {}", g.countdown).unwrap();
    writeln!(&mut lines, "SampleSet: {}", g.sample_set).unwrap();
    writeln!(&mut lines, "StackLeniency: {}", g.stack_leniency).unwrap();
    writeln!(&mut lines, "Mode: {}", g.mode).unwrap();
    writeln!(&mut lines, "LetterboxInBreaks: {}", g.letter_box_in_breaks as u8).unwrap();
    
    if g.use_skin_sprites == true {
        writeln!(&mut lines, "UseSkinSprites: {}", 1).unwrap();
    }

    if g.skin_preference.len() != 0 {
        writeln!(&mut lines, "SkinPreference: {}", g.skin_preference).unwrap();
    }

    if g.epilepsy_warning == true {
        writeln!(&mut lines, "EpilepsyWarning: {}", 1).unwrap();
    }

    if g.countdown_offset != 0 {
        writeln!(&mut lines, "CountdownOffset: {}", g.countdown_offset).unwrap();
    }

    if g.special_style == true {
        writeln!(&mut lines, "SpecialStyle: {}", 1).unwrap();
    }

    writeln!(&mut lines, "WidescreenStoryboard: {}", g.widescreen_storyboard as u8).unwrap();
    writeln!(&mut lines, "SamplesMatchPlaybackRate: {}", g.samples_match_playback_rate as u8).unwrap();

    return lines;
}

pub fn get_editor(e: &Editor) -> String {
    let mut lines = String::new();
    if let Err(e) = writeln!(&mut lines, "[Editor]") {
        println!("{e}");
        return lines
    }

    if e.bookmarks.len() != 0 {
        let mut bookmarks = String::new();

        for (i, item) in e.bookmarks.iter().enumerate() {
            bookmarks.push_str(&item.to_string());
            if i < e.bookmarks.len() - 1 {
                bookmarks.push_str(",");
            }
        }

        writeln!(&mut lines, "Bookmarks: {}", bookmarks).unwrap();
    }

    writeln!(&mut lines, "DistanceSpacing: {}", e.distance_spacing).unwrap();
    writeln!(&mut lines, "BeatDivisor: {}", e.beat_divisor).unwrap();
    writeln!(&mut lines, "GridSize: {}", e.grid_size).unwrap();
    writeln!(&mut lines, "TimelineZoom: {}", e.timeline_zoom).unwrap();

    return lines
}

pub fn get_metadata(m: &Metadata) -> String {
    let mut lines = String::new();
    if let Err(e) = writeln!(&mut lines, "[Metadata]") {
        println!("{e}");
        return lines
    }

    writeln!(&mut lines, "Title: {}", m.title).unwrap();
    writeln!(&mut lines, "TitleUnicode: {}", m.title_unicode).unwrap();
    writeln!(&mut lines, "Artist: {}", m.artist).unwrap();
    writeln!(&mut lines, "ArtistUnicode: {}", m.artist_unicode).unwrap();
    writeln!(&mut lines, "Creator: {}", m.creator).unwrap();
    writeln!(&mut lines, "Version: {}", m.version).unwrap();
    
    if m.source.len() != 0 {
        writeln!(&mut lines, "Source: {}", m.source).unwrap();
    }

    if m.tags.len() != 0 {
        let mut tags = String::new();

        for (i, item) in m.tags.iter().enumerate() {
            tags.push_str(&item.to_string());
            if i < m.tags.len() - 1 {
                tags.push_str(",");
            }
        }

        writeln!(&mut lines, "Tags: {}", tags).unwrap();
    }

    if m.beatmap_id > -1 {
        writeln!(&mut lines, "BeatmapId: {}", m.beatmap_id).unwrap();
    }

    if m.beatmap_set_id > -1 {
        writeln!(&mut lines, "BeatmapSetId: {}", m.beatmap_set_id).unwrap();
    }

    return lines
}

pub fn get_diffuclty(d: &Difficulty) -> String {
    let mut lines = String::new();
    if let Err(e) = writeln!(&mut lines, "[Difficulty]") {
        println!("{e}");
        return lines
    }

    writeln!(&mut lines, "HPDrainRate: {}", d.hp_drain_rate).unwrap();
    writeln!(&mut lines, "CircleSize: {}", d.circle_size).unwrap();
    writeln!(&mut lines, "OverallDifficulty: {}", d.overall_difficulty).unwrap();
    writeln!(&mut lines, "ApproachRate: {}", d.approach_rate).unwrap();
    writeln!(&mut lines, "SliderMultiplier: {}", d.slider_multiplier).unwrap();
    writeln!(&mut lines, "SliderTickRate: {}", d.slider_tick_rate).unwrap();
    return lines
}