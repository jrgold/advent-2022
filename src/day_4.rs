fn pairs_from_file(path: &str) -> Vec<((i32, i32), (i32, i32))> {
    let parse_pair = |s: &str| -> (i32, i32) {
        let (a0, a1) = s.split_once("-").unwrap();
        (str::parse(a0).unwrap(), str::parse(a1).unwrap())
    };
    std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(",").unwrap();
            (parse_pair(a), parse_pair(b))
        }).collect()
}

#[allow(dead_code)]
pub fn part_1() -> i32 {
    pairs_from_file("input/real/4.txt").into_iter()
        .filter(|((a0, a1), (b0, b1))|
            (a0 >= b0 && a1 <= b1) || (a0 <= b0 && a1 >= b1)
        ).count() as i32
}

#[allow(dead_code)]
pub fn part_2() -> i32 {
    pairs_from_file("input/real/4.txt").into_iter()
        .filter(|((a0, a1), (b0, b1))|
            (a0 >= b0 && a0 <= b1) ||
            (a1 >= b0 && a1 <= b1) ||
            (b0 >= a0 && b0 <= a1) ||
            (b1 >= a0 && b1 <= a1)
        ).count() as i32
}
