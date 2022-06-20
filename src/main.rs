mod osu;
use std::time::Instant;

fn main() {
    let filename = String::from(" --- insert here the path to the file --- ");
    let now = Instant::now();
    let beatmap = osu::get_beatmap_from_file(filename);

    let beatmap = match beatmap {
        Ok(beatmap) => beatmap,
        Err(e) => panic!("|| failed to parse beatmap: {}", e)
    };

    // there is a file in the test_files folder that matches these tests
    /*println!("doin tests");
    assert_eq!(beatmap.metadata.artist, "Feryquitous");
    assert_eq!(beatmap.timing_points[0].beat_length, 342.857142857143 as f64);
    assert_eq!(beatmap.timing_points[0].uninherited, true);
    assert_eq!(beatmap.timing_points[0].volume, 25.0);
    assert_eq!(beatmap.hit_objects[0].note_type, 5);
    assert_eq!(beatmap.hit_objects[0].end_time, 0.0);*/
    println!("OK! Elapsed {} milliseconds", now.elapsed().as_millis());
}
