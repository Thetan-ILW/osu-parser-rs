mod osu;
mod magic;
use std::time::Instant;

fn main() {
    let filename = String::from("test_files/beatmap.osu");
    let now = Instant::now();
    let beatmap = osu::import(filename);

    let beatmap = match beatmap {
        Ok(beatmap) => beatmap,
        Err(e) => panic!("|| failed to parse beatmap: {}", e)
    };

    println!("OK! Elapsed {} milliseconds", now.elapsed().as_millis());
    let sliders = &beatmap.note_data.sliders;
    for slider in sliders {
        println!("TIME = {} | LENGTH: {}", slider.time, slider.other.length);
    }
}
