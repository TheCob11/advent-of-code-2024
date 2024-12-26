use core::str;
use std::cmp::Ordering;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_str, updates_str) = input.split_once("\n\n").expect("should have \\n\\n");
    let rules: std::collections::HashSet<(u8, u8)> = rules_str
        .lines()
        .map(|l| {
            (
                l[..2].parse().expect("should be u8"),
                l[3..].parse().expect("should be u8"),
            )
        })
        .collect();
    let res = updates_str
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse().expect("should be u8"))
                .collect::<Vec<u8>>()
        })
        .filter(|l| l.is_sorted_by(|&a, &b| rules.contains(&(a, b))))
        .map(|l| u32::from(l[l.len() / 2]))
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules_str, updates_str) = input.split_once("\n\n").expect("should have \\n\\n");
    let rules: std::collections::HashSet<(u8, u8)> = rules_str
        .lines()
        .map(|l| {
            (
                l[..2].parse().expect("should be u8"),
                l[3..].parse().expect("should be u8"),
            )
        })
        .collect();
    let res: u32 = updates_str
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse().expect("should be u8"))
                .collect::<Vec<u8>>()
        })
        .filter(|l| !l.is_sorted_by(|&a, &b| rules.contains(&(a, b))))
        .map(|mut l| {
            l.sort_unstable_by(|&a, &b| {
                if rules.contains(&(a, b)) {
                    Ordering::Less
                } else if rules.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            u32::from(l[l.len() / 2])
        })
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    #[ignore = "full day"]
    fn part_one_correct() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(3608));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }

    #[test]
    #[ignore = "full day"]
    fn part_two_correct() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(4922));
    }
}
