use std::{collections::{HashSet, hash_map::DefaultHasher}, hash::{Hasher, Hash}};

#[derive(Clone, PartialEq, Eq)]
enum Jet { L, R, }

fn jets_from_file(path: &str) -> Vec<Jet> {
    std::fs::read_to_string(path).unwrap().trim().chars()
        .map(|c| if c == '<' { Jet::L } else { Jet::R })
        .collect()
}

type Coord = (i32, i32);

#[derive(Clone)]
struct Rock { cs: &'static [Coord], h: i32 }

const ROCKS: [Rock; 5] = [
        Rock { cs: &[(0, 0), (1, 0), (2, 0), (3, 0)        ], h: 1},
        Rock { cs: &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)], h: 3},
        Rock { cs: &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], h: 3},
        Rock { cs: &[(0, 0), (0, 1), (0, 2), (0, 3),       ], h: 4},
        Rock { cs: &[(0, 0), (0, 1), (1, 0), (1, 1)        ], h: 2},
];

fn shift_sideways(settled: &HashSet<Coord>, rock: &Rock, x: i32, y: i32, dx: i32) -> i32 {
    let can_move = rock.cs.iter()
        .map(|&(ox, oy)| (x + ox + dx, y + oy))
        .all(|(rx, ry)| rx >= 0 && rx < 7 && !settled.contains(&(rx, ry)));
    if can_move { x + dx } else { x }
}

#[allow(dead_code)]
pub fn part_1() -> i32 {
    let jets = jets_from_file("input/real/17.txt");
    let mut jets = jets.into_iter().cycle();
    let mut rocks = ROCKS.into_iter().cycle();

    let mut settled_rocks: HashSet<Coord> = HashSet::new();
    (0..=6).for_each(|x| { settled_rocks.insert((x, 0)); });

    let mut highest = 0;

    for _ in 0..2022 {
        let mut rock_x = 2;
        let mut rock_y = highest + 4;

        let rock = rocks.next().unwrap();

        loop {
            let jet = jets.next().unwrap();
            let dx = if jet == Jet::L { -1 } else { 1 };

            rock_x = shift_sideways(&settled_rocks, &rock, rock_x, rock_y, dx);

            let drop_obstructed = rock.cs.iter()
                .map(|&(ox, oy)| (rock_x + ox, rock_y + oy - 1))
                .any(|c| settled_rocks.contains(&c));
            if drop_obstructed {
                let drop_obstructed = rock.cs.iter()
                    .map(|&(ox, oy)| (rock_x + ox, rock_y + oy));
                settled_rocks.extend(drop_obstructed);
                highest = highest.max(rock_y + rock.h - 1);
                break;
            } else {
                rock_y -= 1;
            }
        }
    }

    highest
}

fn hash_top_structure(settled: &HashSet<Coord>, top: i32, height: i32) -> u64 {
    let mut h = DefaultHasher::new();
    let mut map: Vec<bool> = vec![];
    for y in top - height..top {
        for x in 0..7 {
            map.push(settled.contains(&(x, y)));
        }
    }
    map.hash(&mut h);
    h.finish()
}

#[allow(dead_code)]
pub fn part_2() -> u64 {
    let jet_list = jets_from_file("input/real/17.txt");
    let mut jets = jet_list.iter().cycle();
    let mut rocks = ROCKS.into_iter().cycle();

    let mut settled_rocks: HashSet<Coord> = HashSet::new();
    (0..=6).for_each(|x| { settled_rocks.insert((x, 0)); });

    let mut highest = 0;

    let mut jets_used = 0;
    let mut rocks_used = 0;

    // jet, rock, hash of structure, height
    let mut patterns: Vec<(usize, usize, u64, i32)> = vec![];

    for _ in 0..10000 {
        let mut rock_x = 2;
        let mut rock_y = highest + 4;

        let rock = rocks.next().unwrap();
        rocks_used += 1;

        loop {
            let jet = jets.next().unwrap();
            jets_used += 1;
            let dx = if *jet == Jet::L { -1 } else { 1 };

            rock_x = shift_sideways(&settled_rocks, &rock, rock_x, rock_y, dx);

            let drop_obstructed = rock.cs.iter()
                .map(|&(ox, oy)| (rock_x + ox, rock_y + oy - 1))
                .any(|c| settled_rocks.contains(&c));
            if drop_obstructed {
                let drop_obstructed = rock.cs.iter()
                    .map(|&(ox, oy)| (rock_x + ox, rock_y + oy));
                settled_rocks.extend(drop_obstructed);
                highest = highest.max(rock_y + rock.h - 1);
                break;
            } else {
                rock_y -= 1;
            }
        }

        let jet_i = jets_used % jet_list.len();
        let rock_i = rocks_used % ROCKS.len();
        let structure = hash_top_structure(&settled_rocks, highest, 20);
        patterns.push((jet_i, rock_i, structure, highest));
    }

    let last = patterns.last().unwrap();
    let (cycle_len_minus_1, prev) = patterns[0..patterns.len()-1].iter().rev()
        .enumerate()
        .find(|&(_, e)| last.0 == e.0 && last.1 == e.1).unwrap();

    let cycle_len = cycle_len_minus_1 + 1;
    let cycle_height = last.3 - prev.3;
    let remaining = 1_000_000_000_000 as u64 - rocks_used as u64;
    let cycles = remaining / cycle_len as u64;
    let remaining = remaining % cycle_len as u64;

    for _ in 0..remaining {
        let mut rock_x = 2;
        let mut rock_y = highest + 4;

        let rock = rocks.next().unwrap();
        rocks_used += 1;

        loop {
            let jet = jets.next().unwrap();
            jets_used += 1;
            let dx = if *jet == Jet::L { -1 } else { 1 };

            rock_x = shift_sideways(&settled_rocks, &rock, rock_x, rock_y, dx);

            let drop_obstructed = rock.cs.iter()
                .map(|&(ox, oy)| (rock_x + ox, rock_y + oy - 1))
                .any(|c| settled_rocks.contains(&c));
            if drop_obstructed {
                let drop_obstructed = rock.cs.iter()
                    .map(|&(ox, oy)| (rock_x + ox, rock_y + oy));
                settled_rocks.extend(drop_obstructed);
                highest = highest.max(rock_y + rock.h - 1);
                break;
            } else {
                rock_y -= 1;
            }
        }
    }

    highest as u64 + cycle_height as u64 * cycles as u64
}
