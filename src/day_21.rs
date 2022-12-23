use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Add, Sub, Mul, Div, Eq
}

#[derive(Debug, Eq, PartialEq)]
enum R {
    N(i64),
    O(String, Op, String),
}

fn riddle_from_file(path: &str) -> HashMap<String, R> {
    std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| {
            let (name, r) = line.split_once(": ").unwrap();
            r.parse()
                .map(|n| (name.to_owned(), R::N(n)))
                .unwrap_or_else(|_| {
                    let op = match r.chars().nth(5).unwrap() {
                        '+' => Op::Add,
                        '-' => Op::Sub,
                        '*' => Op::Mul,
                        '/' => Op::Div,
                        _   => unreachable!(),
                    };
                    (name.to_owned(), R::O(r[0..4].to_owned(), op, r[7..].to_owned()))
                })
        }).collect()
}

fn solve(riddle: &HashMap<String, R>) -> HashMap<String, i64> {
    let mut unfinished: HashSet<String> = HashSet::new();
    let mut solved: HashMap<String, i64> = HashMap::new();
    for (name, r) in riddle.iter() {
        match r {
            R::O(_, _, _) => { unfinished.insert(name.clone()); },
            R::N(n) => { solved.insert(name.clone(), *n); },
        }
    }

    loop {
        let mut newly_finished: Vec<String> = vec![];
        for term_1 in &unfinished {
            let R::O(ref term_2, ref op, ref term_3) = riddle[term_1] else { unreachable!() };

            match (solved.get(term_1), solved.get(term_2), solved.get(term_3)) {
                (None, Some(n_2), Some(n_3)) => {
                    let n_1 = match op {
                        Op::Add => n_2 + n_3,
                        Op::Sub => n_2 - n_3,
                        Op::Mul => n_2 * n_3,
                        Op::Div => n_2 / n_3,
                        Op::Eq => unreachable!(),
                    };
                    solved.insert(term_1.clone(), n_1);
                    newly_finished.push(term_1.clone());
                },
                (Some(n_1), Some(n_2), None) => {
                    let n_3 = match op {
                        Op::Add => n_1 - n_2,
                        Op::Sub => n_2 - n_1,
                        Op::Mul => n_1 / n_2,
                        Op::Div => n_2 / n_1,
                        Op::Eq => unreachable!(),
                    };
                    solved.insert(term_3.clone(), n_3);
                    newly_finished.push(term_1.clone());
                },
                (Some(n_1), None, Some(n_3)) => {
                    let n_2 = match op {
                        Op::Add => n_1 - n_3,
                        Op::Sub => n_1 + n_3,
                        Op::Mul => n_1 / n_3,
                        Op::Div => n_1 * n_3,
                        Op::Eq => unreachable!(),
                    };
                    solved.insert(term_2.clone(), n_2);
                    newly_finished.push(term_1.clone());
                }
                (None, None, Some(n_3)) if *op == Op::Eq => {
                    solved.insert(term_2.clone(), *n_3);
                    newly_finished.push(term_1.clone());
                },
                (None, Some(n_2), None) if *op == Op::Eq => {
                    solved.insert(term_3.clone(), *n_2);
                    newly_finished.push(term_1.clone());
                },
                _ => (),
            }
        }
        if newly_finished.is_empty() {
            break;
        }
        newly_finished.into_iter().for_each(|name| { unfinished.remove(&name); });
    }

    solved
}

#[allow(dead_code)]
pub fn part_1() -> i64 {
    let riddle = riddle_from_file("input/real/21.txt");
    let answers = solve(&riddle);
    answers["root"]
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    let mut riddle = riddle_from_file("input/real/21.txt");
    let R::O(ref term_2, _, ref term_3) = riddle["root"] else { unreachable!(); };
    riddle.insert("root".to_owned(), R::O(term_2.clone(), Op::Eq, term_3.clone()));
    riddle.remove("humn");
    let answers = solve(&riddle);
    answers["humn"]
}
