#[derive(Debug, PartialEq, Eq)]
enum PacketElement {
    L(Vec<PacketElement>),
    I(i32),
}

use std::cmp::Ordering;

use PacketElement::*;

// Some(bool) means we know the answer, None means we need to continue to next element
fn compare_elements(left: &PacketElement, right: &PacketElement) -> Option<bool> {
    match (left, right) {
        (I(l), I(r)) => if l == r { None } else { Some(l < r) }
        (I(l), r@L(_)) => compare_elements(&L(vec![I(*l)]), r),
        (l@L(_), I(r)) => compare_elements(l, &L(vec![I(*r)])),
        (L(l), L(r)) => {
            l.iter().zip(r.iter())
                .find_map(|(l, r)| compare_elements(l, r))
                .or_else(|| {
                    if l.len() == r.len() { None }
                    else { Some(l.len() < r.len()) }
                })
        }
    }
}

fn parse_packet(line: &str) -> PacketElement {
    let mut rest = line;
    let mut stack: Vec<Vec<PacketElement>> = vec![];
    loop {
        match rest.chars().next().unwrap() {
            '[' => {
                stack.push(vec![]);
                rest = &rest[1..];
            },
            ']' => {
                let closed = stack.pop().unwrap();
                if stack.is_empty() {
                    return L(closed);
                } else {
                    stack.last_mut().unwrap().push(L(closed));
                }
                rest = &rest[1..];
            },
            ',' => rest = &rest[1..],
            _ => {
                let num_end = rest.find(&[',', ']']).unwrap();
                let (num, remaining) = rest.split_at(num_end);
                stack.last_mut().unwrap().push(I(num.parse().unwrap()));
                rest = remaining;
            }
        }
    }
}

fn packet_pairs_from_file(path: &str) -> Vec<(PacketElement, PacketElement)> {
    std::fs::read_to_string(path).unwrap()
        .split("\n\n")
        .map(|pair_lines| {
            let (line_1, line_2) = pair_lines.split_once("\n").unwrap();
            (parse_packet(line_1), parse_packet(line_2))
        }).collect()
}

#[allow(dead_code)]
pub fn part_1() -> usize {
    let pairs = packet_pairs_from_file("input/real/13.txt");
    pairs.into_iter()
        .map(|(l, r)| compare_elements(&l, &r))
        .enumerate()
        .filter_map(|(i, b)| if b.unwrap() { Some(i + 1) } else { None })
        .sum()
}

#[allow(dead_code)]
pub fn part_2() -> usize {
    let pairs = packet_pairs_from_file("input/real/13.txt");
    let marker_1 = L(vec![L(vec![I(2)])]);
    let marker_2 = L(vec![L(vec![I(6)])]);
    let mut all_packets: Vec<_> = pairs.into_iter()
        .flat_map(|(l, r)| [l, r].into_iter())
        .chain([
            L(vec![L(vec![I(2)])]),
            L(vec![L(vec![I(6)])])
        ]).collect();
    all_packets.sort_by(|l, r|
        compare_elements(l, r)
            .map(|lt| if lt { Ordering::Less } else { Ordering::Greater })
            .unwrap_or(Ordering::Equal)
    );
    let start = all_packets.iter().position(|e| *e == marker_1).unwrap();
    let end = all_packets.iter().position(|e| *e == marker_2).unwrap();
    (start + 1) * (end + 1)
}
