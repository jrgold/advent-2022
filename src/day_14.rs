use std::collections::HashSet;

fn cave_from_file(path: &str) -> HashSet<(i32, i32)> {
    let mut cave = HashSet::new();

    std::fs::read_to_string(path).unwrap()
        .lines()
        .for_each(|line| {
            let path: Vec<(i32, i32)> = line.split(" -> ")
                .map(|coord| {
                    let (x, y) = coord.split_once(",").unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                }).collect();
            cave.extend(coords_from_path(&path));
        });
    
    cave
}

// includes dupes lol
fn coords_from_path(path: &[(i32, i32)]) -> impl Iterator<Item=(i32, i32)> + '_ {
    path.windows(2)
        .flat_map(|start_and_end| {
            let (x0, y0) = start_and_end[0];
            let (x1, y1) = start_and_end[1];
            let range;
            let f: Box<dyn Fn(i32) -> (i32, i32)>;
            if x0 == x1 {
                range = if y0 < y1 { y0..=y1 } else { y1..=y0 };
                f = Box::new(move |y| (x0, y));
            } else { // y0 == y1
                range = if x0 < x1 { x0..=x1 } else { x1..=x0 };
                f = Box::new(move |x| (x, y0));
            }
            range.map(f)
        })
}

fn add_a_sand_and_see_if_it_falls_into_the_void(cave: &mut HashSet<(i32, i32)>, (mut x, mut y): (i32, i32), bailout_y: i32) -> bool {
    loop {
        if y >= bailout_y {
            return false;
        } else if !cave.contains(&(x, y + 1)) {
            y += 1;
        } else if !cave.contains(&(x - 1, y + 1)) {
            x -= 1;
            y += 1;
        } else if !cave.contains(&(x + 1, y + 1)) {
            x += 1;
            y += 1;
        } else {
            cave.insert((x, y));
            return true;
        }
    }
}

fn add_a_sand_but_now_the_void_is_made_of_floor(cave: &mut HashSet<(i32, i32)>, (mut x, mut y): (i32, i32), floor_y: i32) {
    loop {
        if y + 1 == floor_y {
            cave.insert((x, y));
            return;
        } else if !cave.contains(&(x, y + 1)) {
            y += 1;
        } else if !cave.contains(&(x - 1, y + 1)) {
            x -= 1;
            y += 1;
        } else if !cave.contains(&(x + 1, y + 1)) {
            x += 1;
            y += 1;
        } else {
            cave.insert((x, y));
            return;
        }
    }
}

#[allow(dead_code)]
pub fn part_1() -> i32 {
    let mut cave = cave_from_file("input/real/14.txt");
    let bailout_y = cave.iter().map(|(_, y)| y).max().unwrap() + 1;
    let mut count = 0;

    while add_a_sand_and_see_if_it_falls_into_the_void(&mut cave, (500, 0), bailout_y) {
        count += 1;
    }

    count
}

#[allow(dead_code)]
pub fn part_2() -> i32 {
    let mut cave = cave_from_file("input/real/14.txt");
    let floor_y = cave.iter().map(|(_, y)| y).max().unwrap() + 2;
    let mut count = 0;

    while !cave.contains(&(500, 0)) {
        add_a_sand_but_now_the_void_is_made_of_floor(&mut cave, (500, 0), floor_y);
        count += 1;
    }

    count
}
