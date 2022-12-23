use std::collections::{HashSet, HashMap};

type Position = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Direction { N, S, W, E }
impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::E,
            Direction::E => Direction::N,
        }
    }

    fn iter(&self) -> impl Iterator<Item = Self> {
        std::iter::successors(Some(*self), |d| Some(d.next()))
    }

    fn movement(&self, p: Position) -> Position {
        match self {
            Direction::N => (p.0    , p.1 - 1),
            Direction::S => (p.0    , p.1 + 1),
            Direction::W => (p.0 - 1, p.1    ),
            Direction::E => (p.0 + 1, p.1    ),
        }
    }

    fn need_to_check(&self, p: Position) -> [Position; 3] {
        match self {
            Direction::N => [(p.0 - 1, p.1 - 1), (p.0    , p.1 - 1), (p.0 + 1, p.1 - 1)],
            Direction::S => [(p.0 - 1, p.1 + 1), (p.0    , p.1 + 1), (p.0 + 1, p.1 + 1)],
            Direction::W => [(p.0 - 1, p.1 - 1), (p.0 - 1, p.1    ), (p.0 - 1, p.1 + 1)],
            Direction::E => [(p.0 + 1, p.1 - 1), (p.0 + 1, p.1    ), (p.0 + 1, p.1 + 1)],
        }
    }
}

fn positions_from_file(path: &str) -> HashSet<Position> {
    let mut positions = HashSet::new();
    std::fs::read_to_string(path).unwrap()
        .lines().enumerate()
        .for_each(|(y, line)|
            line.chars().enumerate().for_each(|(x, c)| 
                if c == '#' {
                    positions.insert((x as i32, y as i32));
                } 
            )
        );
    positions
}

fn neighbours(p: Position) -> [Position; 8] {
    [
        (p.0 - 1, p.1 - 1),
        (p.0    , p.1 - 1),
        (p.0 + 1, p.1 - 1),
        (p.0 + 1, p.1    ),
        (p.0 + 1, p.1 + 1),
        (p.0    , p.1 + 1),
        (p.0 - 1, p.1 + 1),
        (p.0 - 1, p.1    ),
    ]
}

fn round(positions: &HashSet<Position>, primary_direction: Direction) -> HashSet<Position> {
    let mut count_of_each_proposal: HashMap<Position, usize> = HashMap::new();
    let mut proposals: HashMap<Position, Position> = HashMap::new();

    'position: for &position in positions {
        for direction in primary_direction.iter().take(4) {
            let neighbours_are_clear = neighbours(position).iter()
                .all(|p| !positions.contains(p));
            if neighbours_are_clear {
                break;
            }
            let direction_is_clear = direction.need_to_check(position).iter()
                .all(|p| !positions.contains(p));
            if direction_is_clear {
                let proposal = direction.movement(position);
                proposals.insert(position, proposal);
                count_of_each_proposal.entry(proposal)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                continue 'position;
            }
        }

        count_of_each_proposal.insert(position, 1);
        proposals.insert(position, position);
    }

    let mut new_positions = HashSet::new();

    for (position, proposal) in proposals {
        if count_of_each_proposal[&proposal] > 1 {
            new_positions.insert(position);
        } else {
            new_positions.insert(proposal);
        }
    }

    new_positions
}

fn area(positions: &HashSet<Position>) -> i32 {
    let min_x = positions.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = positions.iter().map(|&(x, _)| x).max().unwrap();
    let min_y = positions.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = positions.iter().map(|&(_, y)| y).max().unwrap();
    (max_x - min_x + 1) * (max_y - min_y + 1)
}

#[allow(dead_code)]
pub fn part_1() -> i32 {
    let mut positions = positions_from_file("input/real/23.txt");
    for direction in Direction::N.iter().take(10) {
        positions = round(&positions, direction);
    }
    area(&positions) - positions.len() as i32
}

#[allow(dead_code)]
pub fn part_2() -> usize {
    let mut positions = positions_from_file("input/real/23.txt");
    for (i, direction) in Direction::N.iter().enumerate() {
        let new_positions = round(&positions, direction);
        if new_positions == positions {
            return i + 1;
        }
        positions = new_positions;
    }
    unreachable!();
}
