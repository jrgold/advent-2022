#[derive(PartialEq, Eq, Hash)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn loses_against(&self) -> Self {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

    fn from_them_input(c: char) -> RPS {
        match c {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissors,
            _ => unreachable!(),
        }
    }

    fn from_us_input(c: char) -> RPS {
        match c {
            'X' => RPS::Rock,
            'Y' => RPS::Paper,
            'Z' => RPS::Scissors,
            _ => unreachable!(),
        }
    }

    fn selection_score(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

fn strategy_from_file(path: &str) -> Vec<(char, char)> {
    std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| {
            let them = line.as_bytes()[0] as char;
            let us = line.as_bytes()[2] as char;
            (them, us)
        })
        .collect()
}

#[allow(dead_code)]
pub fn part_1() -> i32 {
    strategy_from_file("input/real/2.txt").into_iter()
        .map(|(them, us)| {
            let them = RPS::from_them_input(them);
            let us = RPS::from_us_input(us);
            let round_score = if us.loses_against() == them { 0 }
                else if us == them { 3 }
                else { 6 };
            us.selection_score() + round_score
        })
        .sum()
}

#[allow(dead_code)]
pub fn part_2() -> i32 {
    strategy_from_file("input/real/2.txt").into_iter()
        .map(|(them, outcome)| {
            let them = RPS::from_them_input(them);
            match outcome {
                'X' => 0 + them.wins_against().selection_score(),
                'Y' => 3 + them.selection_score(),
                'Z' => 6 + them.loses_against().selection_score(),
                _ => unreachable!(),
            }
        })
        .sum()
}
