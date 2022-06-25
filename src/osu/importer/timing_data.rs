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

        let time =          split[0].parse().unwrap_or_else(|_|0.0);
        let beat_length =   split[1].parse().unwrap_or_else(|_|0.0);
        let meter =         split[2].parse().unwrap_or_else(|_|0);

        let sample_set =    split[3].parse().unwrap_or_else(|_|0);
        let sample_set =    SampleSet::new(sample_set);

        let sample_index =  split[4].parse().unwrap_or_else(|_|0);
        let volume =        split[5].parse().unwrap_or_else(|_|100.0);
        let uninherited =   split[6].parse().unwrap_or_else(|_|true);
        let effects =       split[7].parse().unwrap_or_else(|_|0);

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