use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let checksum = compute_checksum(INPUT.lines().collect());
    println!("{}", checksum);
    println!("{}", day2_part2(INPUT.lines().collect()));
}

fn compute_checksum(ids: Vec<&str>) -> i64 {
    let mut two_matches = 0;
    let mut three_matches = 0;
    for id in ids {
        let mut occurences = HashMap::with_capacity(26);
        for char in id.chars() {
            *occurences.entry(char).or_insert(0) += 1;
        }
        if occurences.values().any(|&count| count == 2) {
            two_matches += 1;
        }
        if occurences.values().any(|&count| count == 3) {
            three_matches += 1;
        }
    }
    two_matches * three_matches
}

pub fn day2_part2(input: Vec<&str>) -> String {
    for (idx, id) in input.iter().enumerate() {
        for id2 in input.iter().skip(idx + 1) {
            if id.chars().zip(id2.chars()).filter(|(a, b)| a != b).count() == 1 {
                return id
                    .chars()
                    .zip(id2.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect();
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_checksum_correct() {
        let input = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        assert_eq!(12, compute_checksum(input));
    }
}
