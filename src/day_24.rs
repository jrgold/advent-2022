use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction { N, S, E, W, }
type Position = (i32, i32);

// blizzards, start, end, wall corners
fn valley_from_file(path: &str) -> (Vec<(Position, Direction)>, Position, Position, (Position, Position)) {
    let content = std::fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    let mut blizzards = vec![];
    lines.iter().enumerate().for_each(|(y, line)|
        line.chars().enumerate().for_each(|(x, c)|
            match c {
                '^' => blizzards.push(((x as i32, y as i32), Direction::N)),
                'v' => blizzards.push(((x as i32, y as i32), Direction::S)),
                '>' => blizzards.push(((x as i32, y as i32), Direction::E)),
                '<' => blizzards.push(((x as i32, y as i32), Direction::W)),
                _   => (),
            }
        )
    );

    let start_x = lines[0].chars().position(|c| c == '.').unwrap() as i32;
    let end_x = lines.last().unwrap().chars().position(|c| c == '.').unwrap() as i32;

    (blizzards, (start_x, 0), (end_x, lines.len() as i32 - 1), ((0, 0), (lines[0].len() as i32 - 1, lines.len() as i32 - 1)))
}

fn tick_blizzards(blizzards: &[(Position, Direction)], bounds: (Position, Position)) -> (Vec<(Position, Direction)>, HashSet<Position>) {
    let mut new_blizzards = Vec::with_capacity(blizzards.len());
    let mut occupied = HashSet::new();

    for &(mut position, direction) in blizzards {
        let (dx, dy) = match direction {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::E => (1, 0),
            Direction::W => (-1, 0),
        };
        position.0 += dx;
        position.1 += dy;
        if position.0 <= bounds.0.0 {
            position.0 += bounds.1.0 - bounds.0.0 - 1;
        } else if position.0 >= bounds.1.0 {
            position.0 -= bounds.1.0 - bounds.0.0 - 1;
        } else if position.1 <= bounds.0.1 {
            position.1 += bounds.1.1 - bounds.0.1 - 1;
        } else if position.1 >= bounds.1.1 {
            position.1 -= bounds.1.1 - bounds.0.1 - 1;
        }

        new_blizzards.push((position, direction));
        occupied.insert(position);
    }

    (new_blizzards, occupied)
}


fn solve(mut blizzards: Vec<(Position, Direction)>, start: Position, end: Position, bounds: (Position, Position)) -> (i32, Vec<(Position, Direction)>) {
    let mut possible_positions: HashSet<Position> = HashSet::new();
    possible_positions.insert(start);

    for i in 1.. {
        let (new_blizzards, occupied) = tick_blizzards(&blizzards, bounds);
        let mut new_positions = HashSet::new();

        for position in &possible_positions {
            for (dx, dy) in &[(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)] {
                let possible = (position.0 + dx, position.1 + dy);
                if possible == end {
                    return (i, new_blizzards);
                } else if possible != start && (possible.0 <= bounds.0.0 || possible.0 >= bounds.1.0 || possible.1 <= bounds.0.1 || possible.1 >= bounds.1.1) {
                    continue;
                } else if !occupied.contains(&possible) {
                    new_positions.insert(possible);
                }
            }
        }

        blizzards = new_blizzards;
        possible_positions = new_positions;
    }

    unreachable!();
}

#[allow(dead_code)]
pub fn part_1() -> i32 {
    let (blizzards, start, end, bounds) = valley_from_file("input/real/24.txt");

    solve(blizzards, start, end, bounds).0
}

#[allow(dead_code)]
pub fn part_2() -> i32 {
    let (blizzards, start, end, bounds) = valley_from_file("input/real/24.txt");

    let (time_1, blizzards) = solve(blizzards, start, end, bounds);
    let (time_2, blizzards) = solve(blizzards, end, start, bounds);
    let (time_3, _        ) = solve(blizzards, start, end, bounds);

    time_1 + time_2 + time_3
}
