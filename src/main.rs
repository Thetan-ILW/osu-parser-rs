mod osu;
mod convert;
use std::time::Instant;

fn main() {
    let filename = String::from("test_files/ctb.osu");
    let now = Instant::now();
    let beatmap = osu::get_beatmap_from_file(filename);

    let beatmap = match beatmap {
        Ok(beatmap) => beatmap,
        Err(e) => panic!("|| failed to parse beatmap: {}", e)
    };

    println!("OK! Elapsed {} milliseconds", now.elapsed().as_millis());
}
