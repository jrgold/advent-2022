use std::collections::HashSet;

type Cube = (i32, i32, i32);

fn scan_from_file(path: &str) -> Vec<Cube> {
    std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| {
            let mut coords = line.split(',');
            (
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap()
            )
        }).collect()
}

fn set_neighbours(cube: Cube, neighbours: &mut [Cube; 6]) {
    neighbours[0] = (cube.0 - 1, cube.1    , cube.2    );
    neighbours[1] = (cube.0 + 1, cube.1    , cube.2    );
    neighbours[2] = (cube.0    , cube.1 - 1, cube.2    );
    neighbours[3] = (cube.0    , cube.1 + 1, cube.2    );
    neighbours[4] = (cube.0    , cube.1    , cube.2 - 1);
    neighbours[5] = (cube.0    , cube.1    , cube.2 + 1);
}

#[allow(dead_code)]
pub fn part_1() -> usize {
    let scan = scan_from_file("input/real/18.txt");
    let cube_set: HashSet<Cube> = HashSet::from_iter(scan.into_iter());
    let mut neighbours = [(0, 0, 0); 6];
    cube_set.iter()
        .map(|cube| {
            set_neighbours(*cube, &mut neighbours);
            neighbours.iter().filter(|n| !cube_set.contains(n)).count()
        }).sum()
}

fn out_of_bounds(bounds: ((i32, i32), (i32, i32), (i32, i32)), cube: Cube) -> bool {
    cube.0 < bounds.0.0 || cube.0 > bounds.0.1 ||
        cube.1 < bounds.1.0 || cube.1 > bounds.1.1 ||
        cube.2 < bounds.2.0 || cube.2 > bounds.2.1
}

fn classify(
    body: &HashSet<Cube>,
    body_bounds: ((i32, i32), (i32, i32), (i32, i32)),
    inside: &mut HashSet<Cube>,
    outside: &mut HashSet<Cube>,
    classifee: Cube,
) {
    let mut seen: HashSet<Cube> = HashSet::new();
    let mut stack: Vec<Cube> = vec![classifee];

    while !stack.is_empty() {
        let cube = stack.pop().unwrap();
        if body.contains(&cube) {
            continue
        } else if out_of_bounds(body_bounds, cube) {
            seen.insert(cube);
            outside.extend(seen.into_iter());
            return;
        } else if outside.contains(&cube) {
            outside.extend(seen.into_iter());
            return;
        } else if inside.contains(&cube) {
            inside.extend(seen.into_iter());
            return;
        } else {
            let mut neighbours = [(0, 0, 0); 6];
            set_neighbours(cube, &mut neighbours);
            seen.insert(cube);
            stack.extend(neighbours.into_iter().filter(|n| !seen.contains(n)));
        }
    }
    
    // we've flood-filled the space without going out of bounds, must be inside
    inside.extend(seen);
}

#[allow(dead_code)]
pub fn part_2() -> usize {
    let scan = scan_from_file("input/real/18.txt");
    let body: HashSet<Cube> = HashSet::from_iter(scan.into_iter());
    let mut inside: HashSet<Cube> = HashSet::new();
    let mut outside: HashSet<Cube> = HashSet::new();

    let min_x = body.iter().map(|&c| c.0).min().unwrap();
    let max_x = body.iter().map(|&c| c.0).max().unwrap();
    let min_y = body.iter().map(|&c| c.1).min().unwrap();
    let max_y = body.iter().map(|&c| c.1).max().unwrap();
    let min_z = body.iter().map(|&c| c.2).min().unwrap();
    let max_z = body.iter().map(|&c| c.2).max().unwrap();
    let bounds = ((min_x, max_x), (min_y, max_y), (min_z, max_z));

    let mut neighbours = [(0, 0, 0); 6];

    body.iter()
        .for_each(|cube| {
            set_neighbours(*cube, &mut neighbours);
            neighbours.iter().for_each(|&neighbour| {
                classify(
                    &body,
                    bounds,
                    &mut inside,
                    &mut outside,
                    neighbour
                );
            });
        });

    body.iter()
        .map(|cube| {
            set_neighbours(*cube, &mut neighbours);
            neighbours.iter().filter(|n| outside.contains(n)).count()
        }).sum()
}
