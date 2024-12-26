advent_of_code::solution!(3);

#[inline]
fn mul_re() -> Result<regex::Regex, regex::Error> {
    regex::Regex::new(r"(?-u)mul\(\d{1,3},\d{1,3}\)")
}

fn mul_match(m: regex::Match) -> u32 {
    let (l, r) = m.as_str()[4..m.len() - 1]
        .split_once(',')
        .expect("should have ,");
    <u32 as std::ops::Mul>::mul(
        l.parse().expect("should be valid num"),
        r.parse().expect("should be valid num"),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = mul_re()
        .expect("should be valid regex")
        .find_iter(input)
        .map(mul_match)
        .sum();
    Some(res)
}

pub fn part_two_old(input: &str) -> Option<u32> {
    let re = mul_re().expect("should be valid regex");
    // let mut donts = input.match_indices("don't()").map(|(i, _)| i);
    // let mut next_dont = donts.next().expect("should have don't()");
    // let mut dos = input.match_indices("do()").map(|(i, _)| i);
    let mut next_dont = input.find("don't()").expect("should have don't()");

    let mut curr: usize = 0;
    let mut sum: u32 = 0;
    while let Some(m) = re.find(&input[curr..]) {
        if curr + m.start() > next_dont {
            // let Some(next_do) = dos.find(|&i| i > curr + m.start()) else {
            //     break;
            // };
            let Some(next_do) = input[curr + m.start()..]
                .find("do()")
                .map(|i| i + curr + m.start())
            else {
                break;
            };
            curr = next_do;
            // if let Some(x) = donts.find(|&i| i > next_do) {
            if let Some(x) = input[curr..].find("don't()").map(|i| i + curr) {
                next_dont = x;
            } else {
                sum += re.find_iter(&input[curr..]).map(mul_match).sum::<u32>();
                break;
            };
        } else {
            sum += mul_match(m);
            curr += m.end();
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = mul_re().expect("should be valid regex");
    let mut curr: &str = input;
    let mut next_dont = curr.find("don't()").expect("should have don't()");
    let mut sum: u32 = 0;
    while let Some(m) = re.find(curr) {
        if m.start() > next_dont {
            let next_do = if let Some(i) = curr[m.start()..].find("do()") {
                i + m.start()
            } else {
                break;
            };
            curr = &curr[next_do..];
            next_dont = if let Some(i) = curr.find("don't()") {
                i
            } else {
                sum += re.find_iter(curr).map(mul_match).sum::<u32>();
                break;
            };
        } else {
            sum += mul_match(m);
            curr = &curr[m.end()..];
            next_dont -= m.end();
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    #[ignore = "full day"]
    fn part_one_correct() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(189_600_467));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_two_a1() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| 75_541_981 < x));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_two_a2() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| x < 143_145_368));
    }

    #[test]
    #[ignore = "full day"]
    fn part_two_correct() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(107_069_718));
    }
}
