mod osu;
use std::time::Instant;

fn main() {
    let filename = String::from("test_files/slider.osu");
    let now = Instant::now();
    let beatmap = osu::get_beatmap_from_file(filename);

    let beatmap = match beatmap {
        Ok(beatmap) => beatmap,
        Err(e) => panic!("|| failed to parse beatmap: {}", e)
    };

    println!("doin tests");
    assert_eq!(beatmap.hit_objects.sliders[0].other.length, 725 as f64);
    println!("OK! Elapsed {} milliseconds", now.elapsed().as_millis());
}
