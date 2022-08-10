use crate::osu;
use osu::exporter::Export;
use osu::sections::{General, Editor, Metadata, Difficulty};

use std::fmt::Write;
use std::fmt::Error;

impl Export for General {
    fn as_string(&self) -> Result<String, Error> {
        let mut lines = String::new();
        writeln!(&mut lines, "[General]")?;
    
        if self.audio_filename.len() != 0 {
            writeln!(&mut lines, "AudioFilename: {}", self.audio_filename)?;
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
            self.audio_lead_in, 
            self.preview_time,
            self.countdown,
            self.sample_set,
            self.stack_leniency,
            self.mode,
            self.letter_box_in_breaks as u8,
            self.use_skin_sprites as u8,
            self.skin_preference,
            self.epilepsy_warning as u8,
            self.countdown_offset,
            self.special_style as u8,
            self.widescreen_storyboard as u8,
            self.samples_match_playback_rate as u8
        );
    
        writeln!(&mut lines, "{formated}")?;
        return Ok(lines);
    }
}

impl Export for Editor {
    fn as_string(&self) -> Result<String, std::fmt::Error> {
        let mut lines = String::new();
        writeln!(&mut lines, "[Editor]")?;

        if self.bookmarks.len() != 0 {
            let mut bookmarks = String::new();

            for (i, item) in self.bookmarks.iter().enumerate() {
                bookmarks.push_str(&item.to_string());
                if i < self.bookmarks.len() - 1 {
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
            self.distance_spacing,
            self.beat_divisor,
            self.grid_size,
            self.timeline_zoom
        );

        writeln!(&mut lines, "{formated}")?;

        return Ok(lines)
    }
}

impl Export for Metadata {
    fn as_string(&self) -> Result<String, std::fmt::Error> {
        let mut lines = String::new();
        writeln!(&mut lines, "[Metadata]")?;

        let formated = format!(
"Title: {}
TitleUnicode: {}
Artist: {}
ArtistUnicode: {}
Creator: {}
Version: {}",
            self.title,
            self.title_unicode,
            self.artist,
            self.artist_unicode,
            self.creator,
            self.version
        );

        writeln!(&mut lines, "{formated}")?;
        
        if self.source.len() != 0 {
            writeln!(&mut lines, "Source: {}", self.source)?;
        }

        if self.tags.len() != 0 {
            let mut tags = String::new();

            for (i, item) in self.tags.iter().enumerate() {
                tags.push_str(&item.to_string());
                if i < self.tags.len() - 1 {
                    tags.push_str(",");
                }
            }

            writeln!(&mut lines, "Tags: {}", tags)?
        }

        if self.beatmap_id > 0 {
            writeln!(&mut lines, "BeatmapId: {}", self.beatmap_id)?;
        }

        if self.beatmap_set_id > 0 {
            writeln!(&mut lines, "BeatmapSetId: {}", self.beatmap_set_id)?;
        }

        return Ok(lines)
    }
}

impl Export for Difficulty {
    fn as_string(&self) -> Result<String, std::fmt::Error> {
        let mut lines = String::new();
        writeln!(&mut lines, "[Difficulty]")?;

        let formated = format!(
"HPDrainRate: {}
CircleSize: {}
OverallDifficulty: {}
ApproachRate: {}
SliderMultiplier: {}
SliderTickRate: {}",
            self.hp_drain_rate,
            self.circle_size,
            self.overall_difficulty,
            self.approach_rate,
            self.slider_multiplier,
            self.slider_tick_rate
        );

        writeln!(&mut lines, "{formated}")?;
        return Ok(lines)
    }
}