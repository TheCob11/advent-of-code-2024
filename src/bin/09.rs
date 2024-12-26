use std::{cmp::Reverse, collections::BinaryHeap, ops::ControlFlow};

advent_of_code::solution!(9);

const TRIANGULAR_NUMBERS: [usize; 10] = [0, 1, 3, 6, 10, 15, 21, 28, 36, 45];

pub fn part_one(input: &str) -> Option<usize> {
    let map = input
        .as_bytes()
        .iter()
        .map(|b| b - b'0')
        .collect::<Vec<u8>>();

    let curr_end = map.len() / 2;
    let curr_fuel = map[curr_end * 2];
    // pos in expanded disk
    let pos: usize = 0;
    let sum: usize = 0;
    (0..map.len() / 2)
        .try_fold(
            (sum, pos, curr_end, curr_fuel),
            |(mut sum, mut pos, mut curr_end, mut curr_fuel), file_id| {
                if curr_end <= file_id {
                    sum += file_id
                        * (pos * curr_fuel as usize + TRIANGULAR_NUMBERS[curr_fuel as usize - 1]);
                    return ControlFlow::Break(sum);
                }
                let (size, space) = (map[file_id * 2], map[file_id * 2 + 1]);
                sum += file_id * (pos * size as usize + TRIANGULAR_NUMBERS[size as usize - 1]);
                pos += size as usize;
                for j in 0..space {
                    sum += (pos + j as usize) * curr_end;
                    if curr_fuel > 1 {
                        curr_fuel -= 1;
                    } else {
                        if curr_end - 1 <= file_id {
                            return ControlFlow::Break(sum);
                        }
                        curr_end -= 1;
                        curr_fuel = map[curr_end * 2];
                    }
                }
                pos += space as usize;
                ControlFlow::Continue((sum, pos, curr_end, curr_fuel))
            },
        )
        .break_value()
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .as_bytes()
        .iter()
        .map(|b| b - b'0')
        .collect::<Vec<u8>>();
    let gaps_of_size: [BinaryHeap<Reverse<usize>>; 10] = {
        let mut x = std::array::from_fn(|_| BinaryHeap::with_capacity(map.len() / 10));
        let mut pos: usize = 0;
        for i in 0..map.len() / 2 {
            x[map[i * 2 + 1] as usize].push(Reverse(pos));
            pos += map[i * 2] as usize;
        }
        x
    };

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_one_a1() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| x < 24_380_897_290_818));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_one_a2() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| x < 6_386_656_247_039));
    }

    #[test]
    #[ignore = "full day"]
    fn part_one_correct() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6_384_282_079_460));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }

    #[test]
    #[ignore = "full day"]
    fn part_two_correct() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
