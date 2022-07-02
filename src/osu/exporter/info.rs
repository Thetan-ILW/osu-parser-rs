use crate::osu;
use osu::sections::{General, Editor, Metadata, Difficulty};

use std::fmt::Write;
use std::fmt::Error;

pub fn get_general(g: &General) -> Result<String, Error> {
    let mut lines = String::new();
    writeln!(&mut lines, "[General]")?;

    if g.audio_filename.len() != 0 {
        writeln!(&mut lines, "AudioFilename: {}", g.audio_filename)?;
    }

    let formated = format!(
"AudioLeadIn: {}
PreviewTime: {}
Countdown: {}
SampleSet: {}
StackLeniency: {}
Mode: {}
LetterboxInBreaks: {}
UseSkinSprites: {}
SkinPreference: {}
EpilepsyWarning: {}
CountdownOffset: {}
SpecialStyle: {}
WidescreenStoryboard: {}
SamplesMatchPlaybackRate: {}",
        g.audio_lead_in, 
        g.preview_time,
        g.countdown,
        g.sample_set,
        g.stack_leniency,
        g.mode,
        g.letter_box_in_breaks as u8,
        g.use_skin_sprites as u8,
        g.skin_preference,
        g.epilepsy_warning as u8,
        g.countdown_offset,
        g.special_style as u8,
        g.widescreen_storyboard as u8,
        g.samples_match_playback_rate as u8
    );

    writeln!(&mut lines, "{formated}")?;
    return Ok(lines);
}

pub fn get_editor(e: &Editor) -> Result<String, Error> {
    let mut lines = String::new();
    writeln!(&mut lines, "[Editor]")?;

    if e.bookmarks.len() != 0 {
        let mut bookmarks = String::new();

        for (i, item) in e.bookmarks.iter().enumerate() {
            bookmarks.push_str(&item.to_string());
            if i < e.bookmarks.len() - 1 {
                bookmarks.push_str(",");
            }
        }

        writeln!(&mut lines, "Bookmarks: {}", bookmarks)?;
    }

    let formated = format!(
"DistanceSpacing: {}
BeatDivisor: {}
GridSize: {}
TimelineZoom: {}",
        e.distance_spacing,
        e.beat_divisor,
        e.grid_size,
        e.timeline_zoom
    );

    writeln!(&mut lines, "{formated}")?;

    return Ok(lines)
}

pub fn get_metadata(m: &Metadata) -> Result<String, Error> {
    let mut lines = String::new();
    writeln!(&mut lines, "[Metadata]")?;

    let formated = format!(
"Title: {}
TitleUnicode: {}
Artist: {}
ArtistUnicode: {}
Creator: {}
Version: {}",
        m.title,
        m.title_unicode,
        m.artist,
        m.artist_unicode,
        m.creator,
        m.version
    );

    writeln!(&mut lines, "{formated}")?;
    
    if m.source.len() != 0 {
        writeln!(&mut lines, "Source: {}", m.source)?;
    }

    if m.tags.len() != 0 {
        let mut tags = String::new();

        for (i, item) in m.tags.iter().enumerate() {
            tags.push_str(&item.to_string());
            if i < m.tags.len() - 1 {
                tags.push_str(",");
            }
        }

        writeln!(&mut lines, "Tags: {}", tags)?
    }

    if m.beatmap_id > 0 {
        writeln!(&mut lines, "BeatmapId: {}", m.beatmap_id)?;
    }

    if m.beatmap_set_id > 0 {
        writeln!(&mut lines, "BeatmapSetId: {}", m.beatmap_set_id)?;
    }

    return Ok(lines)
}

pub fn get_diffuclty(d: &Difficulty) -> Result<String, Error> {
    let mut lines = String::new();
    writeln!(&mut lines, "[Difficulty]")?;

    let formated = format!(
"HPDrainRate: {}
CircleSize: {}
OverallDifficulty: {}
ApproachRate: {}
SliderMultiplier: {}
SliderTickRate: {}",
        d.hp_drain_rate,
        d.circle_size,
        d.overall_difficulty,
        d.approach_rate,
        d.slider_multiplier,
        d.slider_tick_rate
    );

    writeln!(&mut lines, "{formated}")?;
    return Ok(lines)
}