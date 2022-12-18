use std::collections::{HashSet};

fn sensor_readings_from_file(path: &str) -> Vec<((i32, i32), (i32, i32))> {
    std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| {
            let (sensor_x, rest) = line[12..].split_once(',').unwrap();
            let (sensor_y, rest) = rest[3..].split_once(':').unwrap();
            let (beacon_x, rest) = rest[24..].split_once(',').unwrap();
            let beacon_y = &rest[3..];
            (
                (sensor_x.parse().unwrap(), sensor_y.parse().unwrap()),
                (beacon_x.parse().unwrap(), beacon_y.parse().unwrap()),
            )
        }).collect()
}

pub fn overlap(a: (i32, i32), b: (i32, i32)) -> Option<(i32, i32)> {
    if a.0.max(b.0) <= a.1.min(b.1) {
        Some((a.0.min(b.0), a.1.max(b.1)))
    } else {
        None
    }
}

pub fn merge_zones(zones: &[(i32, i32)]) -> HashSet<(i32, i32)> {
    let mut merged_zones: HashSet<(i32, i32)> = HashSet::new();

    for &z in zones {
        let mut new_zone = z;
        let mut to_remove = vec![];
        for &m in &merged_zones {
            if let Some(merged) = overlap(new_zone, m) {
                to_remove.push(m);
                new_zone = merged;
            }
        }
        to_remove.into_iter().for_each(|r| { merged_zones.remove(&r); });
        merged_zones.insert(new_zone);
    }

    merged_zones
}

#[allow(dead_code)]
pub fn part_1() -> i32 {
    let readings = sensor_readings_from_file("input/real/15.txt");
    let target_y = 2_000_000;
    let sensor_zones: Vec<_> = readings.iter()
        .filter_map(|&((sx, sy), (bx, by))| {
            let distance = (bx - sx).abs() + (by - sy).abs();
            let x_radius = distance - (target_y - sy).abs();
            if x_radius >= 0 {
                Some((sx - x_radius, sx + x_radius))
            } else {
                None
            }
        }).collect();

    let merged = merge_zones(&sensor_zones);

    let beacons_in_merged_at_target_y = readings.iter()
        .filter_map(|&(_, b)|
            if b.1 == target_y { Some(b.0) } else { None }
        ).filter(|&x| {
            merged.iter().any(|&z| z.0 <= x && x <= z.1)
        }).collect::<HashSet<_>>().len() as i32;

    let coverage: i32 = merged.into_iter().map(|z| z.1 - z.0 + 1).sum();

    coverage - beacons_in_merged_at_target_y
}

fn in_range_of_any_sensors(sensors: &[((i32, i32), i32)], point: (i32, i32)) -> bool {
    sensors.iter().any(move |&(centre, range)| (centre.0 - point.0).abs() + (centre.1 - point.1).abs() <= range)
}

fn just_beyond_the_border((centre, range): ((i32, i32), i32)) -> impl Iterator<Item=(i32, i32)> {
    let nw = (0..=range+1).map(move |i| (centre.0 - range - 1 + i, centre.1 + i));
    let ne = (0..=range+1).map(move |i| (centre.0 + range + 1 - i, centre.1 + i));
    let sw = (0..=range+1).map(move |i| (centre.0 - range - 1 + i, centre.1 - i));
    let se = (0..=range+1).map(move |i| (centre.0 + range + 1 - i, centre.1 - i));
    nw.chain(ne).chain(sw).chain(se)
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    let readings = sensor_readings_from_file("input/real/15.txt");
    let border_max = 4_000_000;

    // centre, border distance
    let sensor_zones: Vec<((i32, i32), i32)> = readings.iter()
        .map(|&((sx, sy), (bx, by))| ((sx, sy), (bx - sx).abs() + (by - sy).abs()))
        .collect();
    
    let beacon: (i32, i32) = sensor_zones.iter()
        .flat_map(|&sensor| just_beyond_the_border(sensor))
        .filter(|&(x, y)| x >= 0 && x <= border_max && y >= 0 && y <= border_max)
        .filter(|&point| !in_range_of_any_sensors(&sensor_zones, point))
        .next().unwrap();
    
    beacon.0 as i64 * 4_000_000 + beacon.1 as i64
}
