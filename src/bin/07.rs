advent_of_code::solution!(7);

fn try_div(a: usize, b: usize) -> Option<usize> {
    (a % b == 0).then_some(a / b)
}

fn is_valid(val: usize, terms: &[usize]) -> bool {
    match terms {
        [a, b] => val == a * b || val == a + b,
        [rest @ .., b] => {
            val > *b
                && (try_div(val, *b).is_some_and(|x| is_valid(x, rest)) || is_valid(val - b, rest))
        }
        [] => true,
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, Vec<usize>)> + '_ {
    input.lines().map(|l| {
        let (l, r) = l.split_once(": ").expect("should have \": \"");
        let val: usize = l.parse().expect("should be num");
        let terms: Vec<usize> = r
            .split(' ')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .expect("should be nums");
        (val, terms)
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let res = parse_input(input)
        .filter(|(val, terms)| is_valid(*val, terms))
        .map(|(val, _)| val)
        .sum();
    Some(res)
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn len_b10(a: usize) -> u32 {
    a.ilog10() + 1
}

fn concat(a: usize, b: usize) -> usize {
    a * 10usize.pow(len_b10(b)) + b
}

fn deconcat(a: usize, b: usize) -> Option<usize> {
    try_div(a - b, 10usize.pow(len_b10(b)))
}

fn is_valid_2(val: usize, terms: &[usize]) -> bool {
    match terms {
        [a, b] => val == a + b || val == a * b || val == concat(*a, *b),
        [rest @ .., b] => {
            val > *b
                && (deconcat(val, *b).is_some_and(|x| is_valid_2(x, rest))
                    || try_div(val, *b).is_some_and(|x| is_valid_2(x, rest))
                    || is_valid_2(val - b, rest))
        }
        [] => true,
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let res = parse_input(input)
        .filter(|(val, terms)| is_valid_2(*val, terms))
        .map(|(val, _)| val)
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_one_a1() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| x < 3_166_835_852_141));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_one_a2() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| 1_399_194_355_089 < x));
    }

    #[test]
    #[ignore = "full day"]
    fn part_one_correct() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1_399_219_271_639));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    #[ignore = "full day"]
    fn part_two_correct() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(275_791_737_999_003));
    }
}
