use std::cmp::Ordering;

advent_of_code::solution!(2);

fn parse(input: &str) -> impl Iterator<Item = Vec<u32>> + '_ {
    input.lines().map(|l| {
        l.split(' ')
            .map(str::parse)
            .map(|res| res.expect("should be num"))
            .collect()
    })
}

fn _is_safe_1(record: &[u32]) -> bool {
    fn inner(dir: Ordering, curr: &[u32]) -> bool {
        match curr {
            [] | [_] => true,
            [a, rest @ ..] => rest[0].cmp(a) == dir && rest[0].abs_diff(*a) < 4 && inner(dir, rest),
        }
    }
    let dir = record[1].cmp(&record[0]);
    if dir.is_eq() {
        return false;
    }
    inner(dir, record)
}

fn monotone_within(a: u32, b: u32, dir: Ordering) -> bool {
    b.cmp(&a) == dir && b.abs_diff(a) < 4
}

fn is_safe(record: &[u32]) -> bool {
    if let [] | [_] = record {
        return true;
    }
    let dir = record[1].cmp(&record[0]);
    if dir.is_eq() {
        return false;
    }
    record
        .windows(2)
        .all(|pair| monotone_within(pair[0], pair[1], dir))
}

pub fn part_one(input: &str) -> Option<usize> {
    let res = parse(input).filter(|record| is_safe(record)).count();
    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let res = parse(input)
        .filter(|record| {
            if is_safe(record) || is_safe(&record[1..]) {
                return true;
            }
            let dir = record[1].cmp(&record[0]);
            if dir.is_eq() {
                let mut r2 = record.clone();
                r2.remove(1);
                return is_safe(&r2);
            }
            for i in 1..record.len() - 1 {
                if !monotone_within(record[i], record[i + 1], dir) {
                    let mut r2 = record.clone();
                    r2.remove(i);
                    if is_safe(&r2) {
                        return true;
                    }
                    r2.insert(i, record[i]);
                    r2.remove(i + 1);
                    return is_safe(&r2);
                }
            }
            true
        })
        .count();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    #[ignore = "full day"]
    fn part_one_correct() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(282));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_two_a1() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| 344 < x));
    }

    #[test]
    #[ignore = "full day"]
    fn part_two_correct() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(349));
    }
}
