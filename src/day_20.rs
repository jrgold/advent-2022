fn ciphertext_from_file(path: &str) -> Vec<i64> {
    std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn mix(ciphertext: &[i64], mixes: usize) -> Vec<i64> {
    let mut order_to_value_and_position: Vec<(i64, usize)> = ciphertext.iter().enumerate().map(|(i, &o)| (o, i)).collect();
    let mut value_and_initial_position_buffer = order_to_value_and_position.clone();
    let len = ciphertext.len() as i64;

    for _ in 0..mixes {
        for i in 0..len as usize {
            let current_index = order_to_value_and_position[i].1;
            let mut target_index = current_index as i64 + order_to_value_and_position[i].0;
            while target_index < 0 {
                target_index -= (len - 1) * target_index.div_euclid(len - 1) ;
            }
            while target_index >= len as i64 {
                target_index -= (len - 1) * target_index.div_euclid(len - 1) ;
            }
            let target_index = target_index as usize;
            if (value_and_initial_position_buffer[current_index].0) != order_to_value_and_position[i].0 {
                panic!("oh no our index tracking went to shit somehow");
            }
            let being_mixed = value_and_initial_position_buffer[current_index];
            if current_index < target_index {
                for j in current_index + 1..=target_index {
                    let being_moved = value_and_initial_position_buffer[j];
                    order_to_value_and_position[being_moved.1].1 = j - 1;
                    value_and_initial_position_buffer[j - 1] = being_moved;
                }
            } else if current_index > target_index {
                for j in (target_index..=current_index - 1).rev() {
                    let being_moved = value_and_initial_position_buffer[j];
                    order_to_value_and_position[being_moved.1].1 = j + 1;
                    value_and_initial_position_buffer[j + 1] = being_moved;
                }
            }
            order_to_value_and_position[being_mixed.1].1 = target_index;
            value_and_initial_position_buffer[target_index] = being_mixed;
        }
    }

    value_and_initial_position_buffer.into_iter().map(|(x, _)| x).collect()
}

fn coordinate_sum(plaintext: &[i64]) -> i64 {
    let zero_index = plaintext.iter().position(|&x| x == 0).unwrap();
    let index_1000 = (zero_index + 1000).rem_euclid(plaintext.len());
    let index_2000 = (zero_index + 2000).rem_euclid(plaintext.len());
    let index_3000 = (zero_index + 3000).rem_euclid(plaintext.len());

    plaintext[index_1000] + plaintext[index_2000] + plaintext[index_3000]
}

#[allow(dead_code)]
pub fn part_1() -> i64 {
    let ciphertext = ciphertext_from_file("input/real/20.txt");
    let plaintext = mix(&ciphertext, 1);
    coordinate_sum(&plaintext)
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    let ciphertext = ciphertext_from_file("input/real/20.txt");
    let ciphertext: Vec<_> = ciphertext.into_iter().map(|x| x * 811589153).collect();
    let plaintext = mix(&ciphertext, 10);
    coordinate_sum(&plaintext)
}
