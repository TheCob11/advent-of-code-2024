advent_of_code::solution!(1);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let (mut lefts, mut rights): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|x| {
            let (l, r) = x
                .split_once("   ")
                .expect("Should have the whitespace in between numbers");
            (
                l.parse::<u32>().expect("should be num"),
                r.parse::<u32>().expect("should be num"),
            )
        })
        .unzip();
    lefts.sort_unstable();
    rights.sort_unstable();
    Some(
        lefts
            .into_iter()
            .zip(rights)
            .map(|(l, r)| l.abs_diff(r))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lefts: Vec<u32> = Vec::new();
    let mut freqs: HashMap<u32, u32> = HashMap::new();
    input
        .lines()
        .map(|x| {
            let (l, r) = x
                .split_once("   ")
                .expect("Should have the whitespace in between numbers");
            (
                l.parse().expect("should be num"),
                r.parse().expect("should be num"),
            )
        })
        .for_each(|(l, r)| {
            lefts.push(l);
            *freqs.entry(r).or_insert(0) += 1;
        });
    Some(
        lefts
            .into_iter()
            .map(|x| freqs.get(&x).map_or(0, |n| x * n))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_one_a1() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| x < 14_640_015));
    }

    #[test]
    #[ignore = "full day"]
    fn part_one_correct() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1_580_061));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    #[ignore = "full day"]
    fn part_two_correct() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(23_046_913));
    }
}
