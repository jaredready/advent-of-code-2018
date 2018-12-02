use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let frequency_changes: Vec<_> = contents
        .lines()
        .map(|change| change.parse::<i32>().unwrap())
        .collect();

    println!(
        "Resulting frequency: {}",
        compute_sum_frequency_change(&frequency_changes)
    );
    println!(
        "First repeating frequency: {}",
        find_first_repeating_frequency(&frequency_changes)
    );
}

fn compute_sum_frequency_change(frequency_changes: &Vec<i32>) -> i32 {
    return frequency_changes.iter().sum();
}

fn find_first_repeating_frequency(frequency_changes: &Vec<i32>) -> i32 {
    let mut iteration = 0;
    let mut frequencies: HashSet<i32> = HashSet::new();
    let mut frequency = 0;
    frequencies.insert(frequency);
    while iteration < 1000 {
        for mut x in 0..frequency_changes.len() {
            frequency += frequency_changes.get(x).unwrap();
            if frequencies.contains(&frequency) {
                return frequency;
            } else {
                frequencies.insert(frequency);
            }
            if x == frequency_changes.len() {
                x = 0;
                iteration += 1;
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_sum_frequency_change_correct() {
        assert_eq!(compute_sum_frequency_change(&vec!(1, 2, 3, -1, 4)), 9);
    }

    #[test]
    fn test_find_first_repeating_frequency_correct() {
        assert_eq!(find_first_repeating_frequency(&vec!(7, 7, -2, -7, -4)), 14);
    }
}
