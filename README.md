# osu-parser-rust
practice project, don’t expect much

### Done:
- Importing a file, timing points or hit objects
- General, editor, metadata, difficulty, timing points, colors and hit objects parsing

### Todo:
- Import from bufreader, string, file and path
- Completely redo hit objects
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
let general = &beatmap.info.general;
let metadata = &beatmap.info.metadata;

println!("{}", general.audio_filename);
println!("{} - {}", metadata.artist, metadata.title);
```
Access to notes
```rust
let filename = String::from("test_files/beatmap.osu");
let beatmap = crate::import(&filename);

let mut beatmap = match beatmap {
    Ok(beatmap) => beatmap,
    Err(e) => panic!("🥶 failed to parse beatmap: {}", e),
};

// Getting the very first note
// or you can get any
let note = &beatmap.hit_objects.data[0];

// General data for each type of note can be obtained like this
println!("x: {}", note.x);
println!("y: {}", note.y);
println!("time: {}", note.time);

/*  
    To get unique parameters of different types of notes, 
    you need to check additions type and only after 
    that you can work with their own parameters
*/
match &note.additions {
    Additions::Slider(additions) => {
        println!("params: {}", additions.params);
        println!("length: {}", additions.length);
        println!("slides: {}", additions.slides)
    },
    Additions::Spinner(additions) => {
        println!("end time: {}", additions.end_time)
    },
    Additions::Hold(additions) => {
        println!("end time: {}", additions.end_time)
    }
    _ => {
        // The circle does not have its own unique parameters
    }
}
```
