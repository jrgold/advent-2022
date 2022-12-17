use std::collections::BTreeSet;

fn rucksacks_from_file(path: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn priority(c: char) -> i32 {
    if c.is_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else {
        c as i32 - 'A' as i32 + 27
    }
}

#[allow(dead_code)]
pub fn part_1() -> i32 {
    rucksacks_from_file("input/real/3.txt").iter()
        .map(|rucksack| {
            let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);
            let first_half = BTreeSet::from_iter(first_half.iter().copied());
            let second_half = BTreeSet::from_iter(second_half.iter().copied());
            let &bad_item = (&first_half & &second_half).iter().next().unwrap();
            priority(bad_item)
        })
        .sum()
}

#[allow(dead_code)]
pub fn part_2() -> i32 {
    rucksacks_from_file("input/real/3.txt")
        .chunks(3)
        .map(|trio| trio.into_iter()
            .map(|a| BTreeSet::from_iter(a.into_iter().copied()))
            .reduce(|a, b| &a & &b).unwrap()
            .into_iter().next().unwrap()
        )
        .map(priority)
        .sum()
}
