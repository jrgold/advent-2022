use std::collections::HashMap;

fn heightmap_from_file(path: &str) -> (HashMap<(i32, i32), i32>, (i32, i32), (i32, i32)) {
    let mut start: Option<(i32, i32)> = None;
    let mut end: Option<(i32, i32)> = None;
    let mut heightmap = HashMap::new();

    std::fs::read_to_string(path).unwrap()
        .lines().enumerate()
        .for_each(|(y, line)|
            line.chars().enumerate()
                .for_each(|(x, char)| {
                    let height = match char {
                        'S' => {
                            start = Some((x as i32, y as i32));
                            0
                        },
                        'E' => {
                            end = Some((x as i32, y as i32));
                            25
                        },
                        c   => c as i32 - 'a' as i32,
                    };
                    heightmap.insert((x as i32, y as i32), height);
                })
        );

    (heightmap, start.unwrap(), end.unwrap())
}

pub fn min_distances_to_end(heightmap: &HashMap<(i32, i32), i32>, end: (i32, i32)) -> HashMap<(i32, i32), i32> {
    let max_x = heightmap.keys().map(|&(x, _)| x).max().unwrap();
    let max_y = heightmap.keys().map(|&(_, y)| y).max().unwrap();
    let mut distances_to_end = HashMap::new();
    distances_to_end.insert(end, 0);

    let mut stack = vec![end];

    while let Some(point@(px, py)) = stack.pop() {
        let height = heightmap[&point];
        let distance = distances_to_end[&point];
        let bordering: Vec<_> = [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
            .map(|(dx, dy)| (px + dx, py + dy))
            .filter(|&(x, y)| x >= 0 && x <= max_x && y >= 0 && y <= max_y)
            .filter(|b| heightmap[b] >= height - 1)
            .collect();
        bordering.iter().for_each(|b| {
            let existing_distance = distances_to_end.get(b);
            if existing_distance.map(|&d| distance + 1 < d).unwrap_or(true) {
                distances_to_end.insert(*b, distance + 1);
                stack.push(*b);
            }
        })
    }

    distances_to_end
}

#[allow(dead_code)]
pub fn part_1() -> i32 {
    let (heightmap, start, end) = heightmap_from_file("input/real/12.txt");
    let distances_to_end = min_distances_to_end(&heightmap, end);
    distances_to_end[&start]
}

#[allow(dead_code)]
pub fn part_2() -> i32 {
    let (heightmap, _, end) = heightmap_from_file("input/real/12.txt");
    let distances_to_end = min_distances_to_end(&heightmap, end);
    *heightmap.iter()
        .filter(|(_, h)| **h == 0)
        .filter_map(|(p, _)| distances_to_end.get(p))
        .min().unwrap()
}
