use std::collections::HashSet;

advent_of_code::solution!(4);

#[inline]
fn is_word(w: [u8; 4]) -> bool {
    matches!(&w, b"XMAS" | b"SAMX")
}

// was super close on my messy first try then spent a while cooking this
// monster up to debug what ended up being i > 3 vs i >= 3 for fwd_up
pub fn part_one_disaster(input: &str) -> Option<u32> {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let get_char = |(i, j): (usize, usize)| grid[i][j];
    let fwd_up = |(i, j): (usize, usize)| {
        bool::then(j < grid[0].len() - 3 && i >= 3, || {
            [(i, j), (i - 1, j + 1), (i - 2, j + 2), (i - 3, j + 3)] // gotta be lazy for the sub overflow in debug
        })
    };
    let fwd = |(i, j): (usize, usize)| {
        bool::then_some(
            j < grid[0].len() - 3,
            [(i, j), (i, j + 1), (i, j + 2), (i, j + 3)],
        )
    };
    let fwd_down = |(i, j): (usize, usize)| {
        bool::then_some(
            j < grid[0].len() - 3 && i < grid.len() - 3,
            [(i, j), (i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)],
        )
    };
    let down = |(i, j): (usize, usize)| {
        bool::then_some(
            i < grid.len() - 3,
            [(i, j), (i + 1, j), (i + 2, j), (i + 3, j)],
        )
    };
    let indices: Vec<_> = (0..grid.len())
        .flat_map(|i| (0..grid[0].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| matches!(grid[i][j], b'X' | b'S'))
        .collect();
    let fwd_ups = indices
        .iter()
        .copied()
        .filter_map(fwd_up)
        .filter(|coords| is_word(coords.map(get_char)));
    let fwds = indices
        .iter()
        .copied()
        .filter_map(fwd)
        .filter(|coords| is_word(coords.map(get_char)));
    let fwd_downs = indices
        .iter()
        .copied()
        .filter_map(fwd_down)
        .filter(|coords| is_word(coords.map(get_char)));
    let downs = indices
        .iter()
        .copied()
        .filter_map(down)
        .filter(|coords| is_word(coords.map(get_char)));
    let goods: HashSet<(usize, usize)> = fwd_ups
        .clone()
        .flatten()
        .chain(fwds.clone().flatten())
        .chain(fwd_downs.clone().flatten())
        .chain(downs.clone().flatten())
        .collect();
    let test: Vec<String> = grid
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.iter()
                .enumerate()
                .map(|(j, &b)| {
                    if goods.contains(&(i, j)) {
                        char::from(b)
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect();
    println!("{}", test.join("\n"));
    #[allow(clippy::cast_possible_truncation)]
    Some((fwd_ups.count() + fwds.count() + fwd_downs.count() + downs.count()) as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let count = (0..grid.len())
        .map(|i| {
            (0..grid[0].len())
                .filter(|&j| matches!(grid[i][j], b'X' | b'S'))
                .map(|j|
                    // forward up
                    u32::from(
                        j < grid[0].len() - 3 && i >= 3 &&
                        is_word([grid[i][j], grid[i - 1][j + 1], grid[i - 2][j + 2], grid[i - 3][j + 3]])
                    ) +
                    // forward
                    u32::from(
                        j < grid[0].len() - 3 &&
                        is_word([grid[i][j], grid[i][j + 1], grid[i][j + 2], grid[i][j + 3]])
                    ) +
                    // forward down
                    u32::from(
                        j < grid[0].len() - 3 && i < grid.len() - 3 &&
                        is_word([grid[i][j], grid[i + 1][j + 1], grid[i + 2][j + 2], grid[i + 3][j + 3]])
                    ) +
                    // downward
                    u32::from(
                                                i < grid.len() - 3 &&
                        is_word([grid[i][j], grid[i + 1][j], grid[i + 2][j], grid[i + 3][j]])
                    ))
                .sum::<u32>()
        })
        .sum();
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let count: usize = (1..grid.len() - 1)
        .map(|i| {
            (1..grid[0].len() - 1)
                .filter(|&j| grid[i][j] == b'A')
                .filter(|&j| {
                    matches!(
                        &[
                            grid[i - 1][j - 1],
                            grid[i - 1][j + 1],
                            grid[i + 1][j - 1],
                            grid[i + 1][j + 1]
                        ],
                        b"MSMS" | b"MMSS" | b"SMSM" | b"SSMM"
                    )
                })
                .count()
        })
        .sum();
    #[allow(clippy::cast_possible_truncation)]
    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_one_a1() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| 2362 < x));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_one_a2() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| 2389 > x));
    }

    #[test]
    #[ignore = "full day"]
    fn part_one_correct() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2370));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    #[ignore = "full day"]
    fn part_two_correct() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1908));
    }
}
