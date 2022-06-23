# osu-parser-rust
practice project, don’t expect much

#### Done:
- import,
- general, editor, metadata, difficulty, timing points and hit objects parsing
- and that’s all you need

#### Todo:
- parsing events (ez)
- notedata.rs refactor (never)
- exporting (ez)

#### usage:
```rust
let filename = String::from("/path/to/beatmap.osu");
let beatmap = osu_parser::import(filename);

let beatmap = match beatmap {
    Ok(beatmap) => beatmap,
    Err(e) => panic!("failed to parse beatmap: {}", e)
};

let metadata = &beatmap.settings.metadata;
println!("{} - {} [{}]", metadata.artist, metadata.title, metadata.version);

let sliders = &beatmap.note_data.sliders;
for slider in sliders {
    println!(
        "slider at time: {}, is {} milliseconds long", 
        slider.time, 
        slider.other.length
    );
}
```
