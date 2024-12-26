use std::{collections::HashSet, ops::Index};

advent_of_code::solution!(6);

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Obstacle,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' | '^' => Ok(Tile::Empty),
            '#' => Ok(Tile::Obstacle),
            _ => Err(()),
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Obstacle => '#',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// impl std::ops::Add<Direction> for Coords {
//     type Output = Coords;

//     fn add(self, rhs: Direction) -> Self::Output {
//         use Direction as D;
//         let Coords(i, j) = self;
//         match rhs {
//             D::Up => Coords(i - 1, j),
//             D::Down => Coords(i + 1, j),
//             D::Left => Coords(i, j - 1),
//             D::Right => Coords(i, j + 1),
//         }
//     }
// }

// impl std::ops::AddAssign<Direction> for Coords {
//     fn add_assign(&mut self, rhs: Direction) {
//         *self = *self + rhs;
//     }
// }

impl std::ops::Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use Direction as D;
        match self {
            D::Up => D::Down,
            D::Down => D::Up,
            D::Left => D::Right,
            D::Right => D::Left,
        }
    }
}

impl Direction {
    const fn turn_right(self) -> Direction {
        use Direction as D;
        match self {
            D::Up => D::Right,
            D::Down => D::Left,
            D::Left => D::Up,
            D::Right => D::Down,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coords(usize, usize);
impl<T> Index<Coords> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, Coords(i, j): Coords) -> &Self::Output {
        self.index(i).index(j)
    }
}
impl Coords {
    fn try_index<'a, T>(&self, x: &'a [Vec<T>]) -> Option<&'a T> {
        x.get(self.0).and_then(|r| r.get(self.1))
    }
    fn try_move(self, dir: Direction) -> Option<Self> {
        use Direction as D;
        let Coords(i, j) = self;
        Some(match dir {
            D::Up => Coords(i.checked_sub(1)?, j),
            D::Down => Coords(i.checked_add(1)?, j),
            D::Left => Coords(i, j.checked_sub(1)?),
            D::Right => Coords(i, j.checked_add(1)?),
        })
    }
}

pub fn part_one_original(input: &str) -> Option<u32> {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Tile::try_from(c).expect("should be tile"))
                .collect()
        })
        .collect();
    let mut pos = {
        let i_flat = input.find('^').expect("should have ^");
        Coords(i_flat / (grid.len() + 1), i_flat % (grid[0].len() + 1))
    };
    let mut dir = Direction::Up;
    let mut visited: HashSet<Coords> = HashSet::new();
    visited.insert(pos);

    while let Some((next, tile)) = pos
        .try_move(dir)
        .and_then(|x| x.try_index(&grid).map(|y| (x, y)))
    {
        if let Tile::Obstacle = tile {
            dir = dir.turn_right();
        } else {
            pos = next;
        }
        visited.insert(pos);
    }
    // let test: Vec<String> = grid
    //     .iter()
    //     .enumerate()
    //     .map(|(i, row)| {
    //         row.iter()
    //             .enumerate()
    //             .map(|(j, &x)| {
    //                 if visited.contains(&Coords(i, j)) {
    //                     'X'
    //                 } else {
    //                     char::from(x)
    //                 }
    //             })
    //             .collect()
    //     })
    //     .collect();
    // println!("{}", test.join("\n"));
    #[allow(clippy::cast_possible_truncation)]
    Some(visited.len() as u32)
}

fn parse_grid(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Tile::try_from(c).expect("should be tile"))
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let pos_init = {
        let i_flat = input.find('^').expect("should have ^");
        Coords(i_flat / (grid.len() + 1), i_flat % (grid[0].len() + 1))
    };
    let dir_init = Direction::Up;
    let visited: HashSet<Coords> = walk(pos_init, dir_init, &grid)
        .map(|(pos, _)| pos)
        .collect();
    #[allow(clippy::cast_possible_truncation)]
    Some(visited.len() as u32)
}

#[allow(clippy::type_complexity)]
fn walk(
    pos_init: Coords,
    dir_init: Direction,
    grid: &[Vec<Tile>],
) -> std::iter::Successors<
    (Coords, Direction),
    impl FnMut(&(Coords, Direction)) -> Option<(Coords, Direction)> + '_,
> {
    std::iter::successors(Some((pos_init, dir_init)), |&(pos, dir)| {
        let next = pos.try_move(dir)?;
        if let Tile::Obstacle = next.try_index(grid)? {
            Some((pos, dir.turn_right()))
        } else {
            Some((next, dir))
        }
    })
}

pub fn part_two_naive(input: &str) -> Option<u32> {
    let grid: Vec<Vec<Tile>> = parse_grid(input);
    let start = {
        let i_flat = input.find('^').expect("should have ^");
        Coords(i_flat / (grid.len() + 1), i_flat % (grid[0].len() + 1))
    };
    let possible_obstacles = (0..grid.len()).flat_map(|i| {
        (0..grid[0].len())
            .map(move |j| (i, j))
            .filter(|&(i, j)| matches!(grid[i][j], Tile::Empty) && !(i == start.0 && j == start.1))
    });
    let res = possible_obstacles
        .filter(|&(i, j)| {
            let new_grid = {
                let mut x = grid.clone();
                x[i][j] = Tile::Obstacle;
                x
            };
            let mut history = HashSet::new();
            for x in walk(start, Direction::Up, &new_grid) {
                if !history.insert(x) {
                    return true;
                }
            }
            false
        })
        .count();
    #[allow(clippy::cast_possible_truncation)]
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<Tile>> = parse_grid(input);
    let start = {
        let i_flat = input.find('^').expect("should have ^");
        Coords(i_flat / (grid.len() + 1), i_flat % (grid[0].len() + 1))
    };
    let mut pos = start;
    let mut dir = Direction::Up;
    let mut history: HashSet<(Coords, Direction)> = HashSet::new();
    let mut obstructions: HashSet<Coords> = HashSet::new();
    while let Some((next, tile)) = pos
        .try_move(dir)
        .and_then(|x| Some((x, x.try_index(&grid)?)))
    {
        history.insert((pos, dir));
        let dir_turned = dir.turn_right();
        if let Tile::Obstacle = tile {
            dir = dir_turned;
        } else {
            let mut new_history = HashSet::new();
            let grid_obstructed = {
                let mut x = grid.clone();
                x[pos.0][pos.1] = Tile::Obstacle;
                x
            };
            for x in walk(start, Direction::Up, &grid_obstructed) {
                if !new_history.insert(x) {
                    obstructions.insert(pos);
                    break;
                }
            }
            pos = next;
        }
    }
    // test end
    {
        let mut new_history = history;
        let grid_obstructed = {
            let mut x = grid;
            x[pos.0][pos.1] = Tile::Obstacle;
            x
        };
        for x in walk(
            pos.try_move(-dir).expect("should be able to step back"),
            dir,
            &grid_obstructed,
        ) {
            if !new_history.insert(x) {
                obstructions.insert(pos);
                break;
            }
        }
    }
    // println!("{:?}", obstructions.iter().collect::<Vec<_>>());
    // let test: Vec<String> = grid
    //     .iter()
    //     .enumerate()
    //     .map(|(i, row)| {
    //         row.iter()
    //             .enumerate()
    //             .map(|(j, &x)| {
    //                 if let Some(obs_dir) = obstructions.get(&Coords(i, j)) {
    //                     match obs_dir {
    //                         Direction::Up => 'U',
    //                         Direction::Down => 'D',
    //                         Direction::Left => 'L',
    //                         Direction::Right => 'R',
    //                     }
    //                 } else {
    //                     char::from(x)
    //                 }
    //             })
    //             .collect()
    //     })
    //     .collect();
    // println!("{}", test.join("\n"));
    #[allow(clippy::cast_possible_truncation)]
    Some(obstructions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    #[ignore = "full day"]
    fn part_one_correct() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(5242));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_two_a1() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| 484 < x));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_two_a2() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| x < 1577));
    }

    #[test]
    #[ignore = "single answer"]
    fn part_two_a3() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some_and(|x| x < 1560));
    }

    #[test]
    #[ignore = "full day"]
    fn part_two_correct() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1424));
    }
}
