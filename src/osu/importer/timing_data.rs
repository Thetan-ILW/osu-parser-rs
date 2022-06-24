use crate::osu::SampleSet;
use crate::osu::timing::TimePoint;
use crate::osu::importer::helpers;

pub fn get_timing_points(section: &Vec<String>) -> Vec<TimePoint> {
    let mut timing_points: Vec<TimePoint> = vec!();

    for line in section {
        let split = line.split(",");
        let split = split.collect::<Vec<&str>>();

        if split.len() != 8{
            println!("not valid time point: {line}");
            continue; // If the line is not valid time point array
        }

        let time = helpers::convert(&split[0], 0.0);
        let beat_length = helpers::convert(&split[1], 0.0);
        let meter = helpers::convert(&split[2], 0);

        let sample_set = helpers::convert(&split[3], 0);
        let sample_set = SampleSet::new(sample_set);

        let sample_index = helpers::convert(&split[4], 0);
        let volume = helpers::convert(&split[5], 100.0);
        let uninherited = split[6].parse::<bool>().unwrap_or_else(|_|true); // idiot
        let effects = helpers::convert::<u8>(&split[7], 0);
        let time_point = TimePoint {
            time,
            beat_length,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited,
            effects
        };

        timing_points.push(time_point);
    }

    return timing_points;
}