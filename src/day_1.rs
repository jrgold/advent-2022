fn food_from_file(path: &str) -> Vec<Vec<i32>> {
    std::fs::read_to_string(path).unwrap()
        .split("\n\n")
        .map(|elf| elf
            .lines()
            .map(|cal| str::parse::<i32>(cal).unwrap())
            .collect())
        .collect()
}

#[allow(dead_code)]
pub fn part_1() -> i32 {
    food_from_file("input/real/1.txt").into_iter()
        .map(|elf| elf.iter().sum())
        .max().unwrap()
}

#[allow(dead_code)]
pub fn part_2() -> i32 {
    let mut elf_totals = food_from_file("input/real/1.txt").into_iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<_>>();
    elf_totals.sort_unstable();
    elf_totals[elf_totals.len() - 3..].iter().sum()
}
