use crate::osu::timing::TimePoint;

pub fn get_timing_points(section: &Vec<String>) -> Vec<TimePoint> {
    let mut timing_points: Vec<TimePoint> = vec!();

    for line in section {
        let split = line.split(",");
        let split = split.collect::<Vec<&str>>();

        if split.len() != 8{
            continue; // If the line is not valid time point array
        }

        let time_point = TimePoint::from_split(split);
        timing_points.push(time_point);
    }

    return timing_points;
}