#![allow(clippy::cast_possible_wrap)]
use std::{collections::HashSet, num::TryFromIntError};
advent_of_code::solution!(8);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coords(usize, usize);
impl std::ops::Add for Coords {
    type Output = Self;

    fn add(self, Coords(i, j): Self) -> Self::Output {
        Coords(self.0 + i, self.1 + j)
    }
}

impl std::ops::Sub for Coords {
    type Output = (isize, isize);

    fn sub(self, Coords(i, j): Self) -> Self::Output {
        (self.0 as isize - i as isize, self.1 as isize - j as isize)
    }
}

impl TryFrom<(isize, isize)> for Coords {
    type Error = TryFromIntError;

    fn try_from((i, j): (isize, isize)) -> Result<Self, Self::Error> {
        Ok(Coords(i.try_into()?, j.try_into()?))
    }
}

impl std::ops::Add<(isize, isize)> for Coords {
    type Output = (isize, isize);

    fn add(self, (i, j): (isize, isize)) -> Self::Output {
        (self.0 as isize + i, self.1 as isize + j)
    }
}

impl std::ops::Sub<(isize, isize)> for Coords {
    type Output = (isize, isize);

    fn sub(self, (i, j): (isize, isize)) -> Self::Output {
        (self.0 as isize - i, self.1 as isize - j)
    }
}

impl Coords {
    // fn try_sub(self, rhs: (isize, isize)) -> Option<Self> {
    //     Some(Coords(i.try_into().ok()?, j.try_into().ok()?))
    // }
    // fn max_sub(self, Coords(i, j): Coords) -> Coords {
    //     Coords(self.0.abs_diff(i), self.1.abs_diff(j))
    // }
    fn within(&self, bounds: (usize, usize)) -> bool {
        self.0 < bounds.0 && self.1 < bounds.1
    }
    fn try_new(x: (isize, isize), bounds: (usize, usize)) -> Option<Coords> {
        x.try_into().ok().filter(|x| Coords::within(x, bounds))
    }
}

type Tile = Option<char>;

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '.' { None } else { Some(c) })
                .collect()
        })
        .collect()
}

fn antinode_pair(a: Coords, b: Coords) -> [Option<Coords>; 2] {
    let d = b - a;
    [(a - d).try_into().ok(), (b + d).try_into().ok()]
}

#[allow(dead_code)]
fn positions_pretty<'a>(
    grid: impl IntoIterator<Item = &'a Vec<Tile>>,
    positions: &HashSet<Coords>,
) -> String {
    grid.into_iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, tile)| {
                    if positions.contains(&Coords(i, j)) {
                        '#'
                    } else {
                        tile.unwrap_or('.')
                    }
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    // seems to not be an issue that this is iterating again, either the compilier is figuring it out
    // or its just negligible
    let antannae: Vec<Coords> = (0..grid.len())
        .flat_map(|i| {
            (0..grid[0].len())
                .map(move |j| Coords(i, j))
                .filter(|&Coords(i, j)| grid[i][j].is_some())
        })
        .collect();
    let positions: HashSet<Coords> = antannae[..antannae.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(iter, p1)| {
            antannae[iter + 1..]
                .iter()
                .filter(|&&Coords(i, j)| grid[p1.0][p1.1] == grid[i][j])
                .flat_map(|&p2| antinode_pair(*p1, p2))
                .filter_map(|x| x.filter(|p| p.within((grid.len(), grid[0].len()))))
        })
        .collect();
    // println!("{}", positions_pretty(&grid, &positions));
    #[allow(clippy::cast_possible_truncation)]
    Some(positions.len() as u32)
}

fn antinodes_all(a: Coords, b: Coords, bounds: (usize, usize)) -> impl Iterator<Item = Coords> {
    let d = b - a;
    let backwards = std::iter::successors(Some(a), move |&curr| Coords::try_new(curr - d, bounds));
    let forwards = std::iter::successors(Some(b), move |&curr| Coords::try_new(curr + d, bounds));
    backwards.chain(forwards)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    // seems to not be an issue that this is iterating again, either the compilier is figuring it out
    // or its just negligible
    let antannae: Vec<Coords> = (0..grid.len())
        .flat_map(|i| {
            (0..grid[0].len())
                .map(move |j| Coords(i, j))
                .filter(|&Coords(i, j)| grid[i][j].is_some())
        })
        .collect();
    let bounds = (grid.len(), grid[0].len());
    let positions: HashSet<Coords> = antannae[..antannae.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(iter, p1)| {
            antannae[iter + 1..]
                .iter()
                .filter(|&&Coords(i, j)| grid[p1.0][p1.1] == grid[i][j])
                .flat_map(|&p2| antinodes_all(*p1, p2, bounds))
        })
        .collect();
    // println!("{}", positions_pretty(&grid, &positions));
    #[allow(clippy::cast_possible_truncation)]
    Some(positions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_one_a1() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| x < 1978));
    }

    #[test]
    #[ignore = "full day"]
    fn part_one_correct() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(327));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
    #[test]
    #[ignore = "full day"]
    fn part_two_correct() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1233));
    }
}
