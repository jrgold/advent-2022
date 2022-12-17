use std::{collections::{HashMap, VecDeque}};

// Flow rates, adjacencies, starter node
fn read_valve_network(path: &str) -> (Vec<i32>, Vec<Vec<usize>>, usize) {
    let data: Vec<_> = std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(&['=',';']).collect();
            let name = parts[0][6..8].to_owned();
            let flow_rate: i32 = parts[1].parse().unwrap();
            let tunnels: Vec<_> = if parts[2].as_bytes()[22] as char == 's' {
                parts[2][24..].split(", ").map(str::to_owned).collect()
            } else {
                parts[2][23..].split(", ").map(str::to_owned).collect()
            };
            (name, flow_rate, tunnels)
        }).collect();
    
    let names_to_index: HashMap<_, _> = data.iter().enumerate().map(|(i, (name, _, _))| (name.clone(), i)).collect();
    let adjacencies: Vec<Vec<usize>> = data.iter()
        .map(|(_, _, adjacents)|
            adjacents.iter().map(|t| names_to_index[t]).collect()
        ).collect();
    let rates: Vec<i32> = data.iter().map(|(_, rate, _)| *rate).collect();

    (rates, adjacencies, names_to_index["AA"])
}

fn shortest_path_lengths(adjacencies: &[Vec<usize>], nodes_of_interest: &[usize]) -> HashMap<(usize, usize), usize> {
    let mut paths = HashMap::new();
    for (i, &start) in nodes_of_interest.iter().enumerate() {
        for &end in &nodes_of_interest[i+1..] {
            let shortest_path_length = get_a_shortest_path(adjacencies, start, end).unwrap().len() - 1; // don't count the start node
            paths.insert((start, end), shortest_path_length);
            paths.insert((end, start), shortest_path_length);
        }
    }
    paths
}

fn get_a_shortest_path(neighbours: &[Vec<usize>], start: usize, target: usize) -> Option<Vec<usize>> {
    let mut path_stack: VecDeque<Vec<usize>> = VecDeque::new();
    path_stack.push_back(vec![start]);

    loop {
        let Some(mut path) = path_stack.pop_front() else { return None; };
        let end = *path.last().unwrap();
        for &next in &neighbours[end] {
            if next == target {
                path.push(next);
                return Some(path);
            }
            if !path.contains(&next) {
                let mut option = path.clone();
                option.push(next);
                path_stack.push_back(option);
            }
        }
    }
}

fn lookup_array_from_pair_map<T: Clone>(map: HashMap<(usize, usize), T>, size: usize, default: T) -> Vec<T> {
    let mut array = vec![default.clone(); size * size];
    for ((start, end), value) in map.into_iter() {
        array[size * start + end] = value
    }
    array
}

fn biggest_vent_pressure(
    rates: &[i32],              // flow rates
    path_lengths_array: &[usize],  // distance between nodes. indexed [node1 * nodecount + node2]
    v: usize,                   // current node
    tick: i32,                  // current minute, counts down from start
    total: i32,                 // total impact of vents opened so far
    remaining_mask: u64,        // bitmask of useful vents that are still closed
    max: &mut i32,              // pointer to store highest total impact
) {
    let mut in_progress_mask = remaining_mask;
    while in_progress_mask.leading_zeros() < 64 {
        let next_vent = 63 - in_progress_mask.leading_zeros();
        in_progress_mask &= !(1 << next_vent);
        let distance = if next_vent == v as u32 { 0 }
            else { path_lengths_array[v * rates.len() + next_vent as usize] };
        let new_tick = tick - distance as i32 - 1;
        if new_tick > 0 {
            let new_total = total + new_tick * rates[next_vent as usize];
            let new_mask = remaining_mask & !(1 << next_vent);
            if remaining_mask == 0 {
                *max = (*max).max(total);
            } else {
                biggest_vent_pressure(rates, path_lengths_array, next_vent as usize, new_tick, new_total, new_mask, max);
            }
        } 
    }
    *max = (*max).max(total);
}

fn build_vent_set_pressures(
    target_mask: u64,                       // bitmask of all the useful vents
    rates: &[i32],                          // flow rates
    path_lengths_array: &[usize],              // distance between nodes. indexed [node1 * nodecount + node2]
    v: usize,                               // current node
    tick: i32,                              // current minute, counts down from start
    total: i32,                             // total impact of vents opened so far
    remaining_mask: u64,                    // bitmask of useful vents that are still closed
    vent_options: &mut HashMap<u64, i32>,   // complete map of possible (non-optimal) vent sets used to maximum impact
) {
    let mut in_progress_mask = remaining_mask;
    vent_options.entry(target_mask & !remaining_mask)
        .and_modify(|e| *e = (*e).max(total))
        .or_insert(total);
    while in_progress_mask.leading_zeros() < 64 {
        let next = 63 - in_progress_mask.leading_zeros();
        in_progress_mask &= !(1 << next);
        let distance = if next == v as u32 { 0 }
            else { path_lengths_array[v * rates.len() + next as usize] };
        let new_tick = tick - distance as i32 - 1;
        if new_tick > 0 {
            let new_total = total + new_tick * rates[next as usize];
            let new_mask = remaining_mask & !(1 << next);
            if remaining_mask == 0 {
                vent_options.entry(target_mask & !remaining_mask)
                    .and_modify(|e| *e = (*e).max(total))
                    .or_insert(total);
            } else {
                build_vent_set_pressures(target_mask, rates, path_lengths_array, next as usize, new_tick, new_total, new_mask, vent_options);
            }
        } 
    }
}

fn fastest_team_effort(vent_set_pressures: &HashMap<u64, i32>) -> i32 {
    let array: Vec<_> = vent_set_pressures.iter().collect();

    let mut max = 0;
    for (i, &(mask_1, max_1)) in array.iter().enumerate() {
        for &(mask_2, max_2) in &array[i + 1..] {
            if mask_1 & mask_2 == 0 {
                max = max.max(max_1 + max_2);
            }
        }
    }
    max
}

#[allow(dead_code)]
pub fn part_1() -> i32 {
    let (rates, adjacencies, starter) = read_valve_network("input/real/16.txt");

    // Find the shortest distances between nodes of interest (non-zero flow-rate valves and the starting valve)
    let targets_plus_starter: Vec<_> = rates.iter().enumerate().filter(|&(i, r)| *r > 0 || i == starter).map(|(i, _)| i).collect();
    let path_lengths = shortest_path_lengths(&adjacencies, &targets_plus_starter);
    let path_lengths_array = lookup_array_from_pair_map(path_lengths, rates.len(), 0);

    // The set of valves that we're interested in visiting (non-zero flow-rate)
    let target_mask: u64 = rates.iter().enumerate().filter(|&(_, r)| *r > 0).map(|(i, _)| 1 << i).sum();

    // Visit every order of target valves we can in 30 minutes, saving the largest vent result we see
    let mut max = 0;
    biggest_vent_pressure(&rates, &path_lengths_array, starter, 30, 0, target_mask, &mut max);
    max
}

#[allow(dead_code)]
pub fn part_2() -> i32 {
    let (rates, adjacencies, starter) = read_valve_network("input/real/16.txt");

    // Find the shortest distances between nodes of interest (non-zero flow-rate valves and the starting valve)
    let targets_plus_starter: Vec<_> = rates.iter().enumerate().filter(|&(i, r)| *r > 0 || i == starter).map(|(i, _)| i).collect();
    let path_lengths = shortest_path_lengths(&adjacencies, &targets_plus_starter);
    let path_lengths_array = lookup_array_from_pair_map(path_lengths, rates.len(), 0);

    // The set of valves that we're interested in visiting (non-zero flow-rate)
    let target_mask: u64 = rates.iter().enumerate().filter(|&(_, r)| *r > 0).map(|(i, _)| 1 << i).sum();

    // Visit every order of target valves we can in 26 minutes (**including** partial combinations - important),
    // saving the max vent volume possible for each set of target valves
    let mut vent_set_pressures: HashMap<u64, i32> = HashMap::new();
    build_vent_set_pressures(target_mask, &rates, &path_lengths_array, starter, 26, 0, target_mask, &mut vent_set_pressures);

    // Find the two non-intersecting vent sets with the highest total vent volume
    // Non-intersecting vent sets enforces that the elephant and I don't open the same valve
    fastest_team_effort(&vent_set_pressures)
}
