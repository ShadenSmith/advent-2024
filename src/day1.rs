use std::iter::zip;

use std::fs;
use std::io::{BufRead, BufReader};

fn total_distance(left: &mut [u64], right: &mut [u64]) -> u64 {
    left.sort();
    right.sort();

    zip(left.iter(), right.iter())
        .map(|(l, r)| l.max(r) - l.min(r))
        .sum()
}

pub fn run_a(filename: &str) -> u64 {
    let reader = BufReader::new(fs::File::open(filename).expect("Could not open input"));

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let vals: Vec<u64> = line
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        assert_eq!(vals.len(), 2);

        left.push(vals[0]);
        right.push(vals[1]);
    }

    total_distance(&mut left, &mut right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_distance() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(total_distance(&mut left, &mut right), 11)
    }
}
