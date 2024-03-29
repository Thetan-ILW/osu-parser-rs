/*
    WELCOME TO HELL!
*/

#[cfg(test)]
mod tests {
    use crate::osu::note::Additions;

    #[test]
    fn import_beatmap() {
        let filename = String::from("test_files/beatmap.osu");
        let beatmap = crate::import(&filename);

        let beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("🥶 failed to parse beatmap: {}", e),
        };

        assert_eq!(beatmap.info.general.preview_time, -69.0);
        assert_eq!(beatmap.info.difficulty.approach_rate, 6.9 as f32);
        assert_eq!(beatmap.info.general.letter_box_in_breaks, false);
        assert_eq!(beatmap.info.general.samples_match_playback_rate, true);
        assert_eq!(beatmap.timing_points.data[0].time, 999.0);
        
        let additions = &beatmap.hit_objects.data[3].additions;

        match additions {
            Additions::Slider(slider) => {
                assert_eq!(slider.length, 500.0)
            }
            _ => panic!("wrong index lol")
        }

        println!("{}", beatmap.info.events.data[0].params[0]);
    }
    #[test]
    fn import_only_hit_objects() {
        let filename = String::from("test_files/refactor.osu");
        let hit_objects = crate::import_section(&filename, crate::Section::HitObjects(None));

        let _ = match hit_objects {
            Ok(hit_objects) => hit_objects,
            Err(e) => panic!("🥶 failed to parse beatmap: {}", e),
        };

        let filename = String::from("test_files/refactor-spinnerz.osu");
        let _ = crate::import_section(&filename, crate::Section::HitObjects(None));

        let filename = String::from("test_files/refactor-holds.osu");
        let _ = crate::import_section(&filename, crate::Section::HitObjects(None));
    }
    #[test]
    fn color_test() {
        let filename = String::from("test_files/colortest.osu");
        let beatmap = crate::import(&filename);

        let beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("🥶 failed to parse beatmap: {}", e),
        };

        let color = beatmap.info.colors.data[0].clone();

        assert_eq!(color.0, 69);
        assert_eq!(color.1, 228);
        assert_eq!(color.2, 13);
    }
    #[test]
    fn open_blank_beatmap() {
        let filename = String::from("test_files/blank.osu");
        let beatmap = crate::import(&filename);

        let _beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("|| failed to parse beatmap: {}", e),
        };
    }

    #[test]
    fn import_and_export() {
        let filename = String::from("test_files/beatmap.osu");
        let now = std::time::Instant::now();
        let beatmap = crate::import(&filename);

        let mut beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("🥶 failed to parse beatmap: {}", e),
        };

        let import_time = now.elapsed().as_millis();
        println!("Imported in {}", import_time);

        assert_eq!(beatmap.info.events.data[0].params[0], "kody_title.jpg".to_string());

        beatmap.info.metadata.version = "exported".to_string();
        let result = crate::export("test_files/new.osu", &beatmap);
        match result {
            Ok(_) => println!("success"),
            Err(e) => panic!("uhhh ummm {e}")
        }
        
        println!("Exported in {}", now.elapsed().as_millis() - import_time)
    }

    #[test]
    fn import_broken_beatmap() {
        let filename = String::from("test_files/broken.osu");
        let beatmap = crate::import(&filename);

        let beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("|| failed to parse beatmap: {}", e),
        };


    }
}
