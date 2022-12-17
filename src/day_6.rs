use std::collections::HashMap;

fn datastream_from_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

#[allow(dead_code)]
pub fn part_1() -> usize {
    datastream_from_file("input/real/6.txt").as_bytes()
        .windows(4).enumerate()
        .find(|(_index, chars)|
            chars[0] != chars[1] && chars[0] != chars[2] && chars[0] != chars[3] &&
            chars[1] != chars[2] && chars[1] != chars[3] &&
            chars[2] != chars[3]
        ).map(|(index, _)| index).unwrap()
        + 4
}

pub fn find_marker(chars: &[u8], marker_size: usize) -> usize {
    let mut map = HashMap::<u8, usize>::new();
    let insert = |m: &mut HashMap<u8, usize>, c: u8| {
        m.entry(c).and_modify(|x| *x += 1).or_insert(1);
    };
    let remove = |m: &mut HashMap<u8, usize>, c: u8| {
        let entry = m.entry(c).and_modify(|x| *x -= 1).or_insert(0);
        if *entry == 0 { m.remove(&c); }
    };

    chars.iter().take(marker_size).for_each(|&c|
        insert(&mut map, c)
    );
    if map.len() == marker_size { return marker_size }
    for (index, (&new, &old)) in chars.iter().skip(marker_size).zip(chars.iter()).enumerate() {
        remove(&mut map, old);
        insert(&mut map, new);
        if map.len() == marker_size {
            return index + marker_size + 1;
        }
    }
    unreachable!();
}

#[allow(dead_code)]
pub fn part_2() -> usize {
    // datastream_from_file("input/real/6.txt").as_bytes()
    //     .windows(14).enumerate()
    //     .find(|(_index, chars)|
    //         HashSet::<u8>::from_iter(chars.iter().copied()).len() == 14
    //     ).map(|(index, _)| index).unwrap()
    //     + 14

    let data = datastream_from_file("input/real/6.txt");
    find_marker(data.as_bytes(), 14)
}
