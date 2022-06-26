# osu-parser-rust
practice project, don’t expect much

### Done:
- Importing a file, timing points or hit objects
- General, editor, metadata, difficulty, timing points, colors and hit objects parsing

### Todo:
- Parsing events
- Refactor notedata.rs (never)
- Exporting
- Multithreading (we need more speed!)
____
## Usage:
Add this to your Cargo.toml file:
```toml
[dependencies]
osu-parser = { git = "https://github.com/Righeil/osu-parser-rust" }
```
Import the beatmap like this:

```rust
let filename = String::from("/path/to/beatmap.osu");
let beatmap = osu_parser::import(filename);

let beatmap = match beatmap {
    Ok(beatmap) => beatmap,
    Err(e) => panic!("failed to parse beatmap: {}", e)
};
```
Access to metadata fields
```rust
let general = &beatmap.settings.general;
let metadata = &beatmap.settings.metadata;

println!("{}", general.audio_filename);
println!("{} - {}", metadata.artist, metadata.title);
```
Access to notes
```rust
let circle = &beatmap.note_data.circles[0];
let circle_x = circle.x;
let circle_y = circle.y;

let slider = &beatmap.note_data.sliders[0];
let slider_time = slider.time;
let slider_length = slider.other.length;
```
