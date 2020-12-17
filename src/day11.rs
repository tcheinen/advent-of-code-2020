use itertools::Itertools;
use nom::lib::std::fmt::Formatter;
use std::fmt::{Display, Write};

/// https://adventofcode.com/2020/day/11
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Seat {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatLayout {
    pub seats: Vec<Vec<Seat>>,
    pub width: usize,
    pub height: usize,
}

impl Display for SeatLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in &self.seats {
            for y in x {
                f.write_char(match y {
                    Seat::Occupied => '#',
                    Seat::Empty => 'L',
                    Seat::Floor => '.',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn get_adjacent(seats: &[Vec<Seat>], x: isize, y: isize) -> Vec<Seat> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .filter(|(a, b)| *a >= 0 && *b >= 0)
    .map(|(a, b)| (a as usize, b as usize))
    .filter_map(|(a, b)| seats.get(a)?.get(b))
    .cloned()
    .collect_vec()
}

fn get_adjacent_with_sight(seats: &[Vec<Seat>], x: isize, y: isize) -> Vec<Seat> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .filter(|(a, b)| {
        *a >= 0 && *b >= 0 && *a < seats.len() as isize && *b < seats[0].len() as isize
    })
    .map(|(a, b)| (a - x, b - y, a, b))
    .filter_map(|(inc_x, inc_y, a, b)| {
        let first = std::iter::repeat((a, b))
            .enumerate()
            .map(|(index, (c, d))| (c + (index as isize * inc_x), d + (index as isize * inc_y)))
            .take_while(|(c, d)| {
                *c >= 0 && *d >= 0 && *c < seats.len() as isize && *d < seats[0].len() as isize
            })
            .find(|(c, d)| {
                seats.get(*c as usize).unwrap().get(*d as usize).unwrap() != &Seat::Floor
            })?;
        seats.get(first.0 as usize)?.get(first.1 as usize)
    })
    .cloned()
    .collect_vec()
}

impl SeatLayout {
    pub fn tick_part1(&mut self) -> bool {
        let original = self.seats.clone();
        let mut changed = false;
        for (x, y) in (0..self.height).cartesian_product(0..self.width) {
            changed |= match self.seats[x][y] {
                Seat::Occupied => {
                    let change = get_adjacent(&original, x as isize, y as isize)
                        .iter()
                        .filter(|x| **x == Seat::Occupied)
                        .count()
                        >= 4;
                    if change {
                        self.seats[x][y] = Seat::Empty;
                    }
                    change
                }
                Seat::Empty => {
                    let change = get_adjacent(&original, x as isize, y as isize)
                        .iter()
                        .filter(|x| **x == Seat::Occupied)
                        .count()
                        == 0;
                    if change {
                        self.seats[x][y] = Seat::Occupied;
                    }
                    change
                }
                Seat::Floor => false,
            };
        }
        changed
    }
    pub fn tick_part2(&mut self) -> bool {
        let original = self.seats.clone();
        let mut changed = false;
        for (x, y) in (0..self.height).cartesian_product(0..self.width) {
            changed |= match self.seats[x][y] {
                Seat::Occupied => {
                    let change = get_adjacent_with_sight(&original, x as isize, y as isize)
                        .iter()
                        .filter(|x| **x == Seat::Occupied)
                        .count()
                        >= 5;
                    if change {
                        self.seats[x][y] = Seat::Empty;
                    }
                    change
                }
                Seat::Empty => {
                    let change = get_adjacent_with_sight(&original, x as isize, y as isize)
                        .iter()
                        .filter(|x| **x == Seat::Occupied)
                        .count()
                        == 0;
                    if change {
                        self.seats[x][y] = Seat::Occupied;
                    }
                    change
                }
                Seat::Floor => false,
            };
        }
        changed
    }
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> SeatLayout {
    SeatLayout {
        width: input.lines().nth(0).unwrap_or("").chars().count(),
        height: input.lines().count(),
        seats: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|seat| match seat {
                        '.' => Seat::Floor,
                        'L' => Seat::Empty,
                        '#' => Seat::Occupied,
                        _ => panic!("logic error smh"),
                    })
                    .collect_vec()
            })
            .collect_vec(),
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &SeatLayout) -> usize {
    let mut seats = input.clone();
    loop {
        if !seats.tick_part1() {
            break;
        }
    }
    seats
        .seats
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x == Seat::Occupied)
        .count()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &SeatLayout) -> usize {
    let mut seats = input.clone();
    loop {
        if !seats.tick_part2() {
            break;
        }
    }
    seats
        .seats
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x == Seat::Occupied)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_adjacent() {
        let provided = "LLL\nLLL\nLLL";
        let adjacent = get_adjacent(&generator(provided).seats, 1, 1);
        assert_eq!(8, adjacent.len());
    }

    #[test]
    fn it_gets_adjacent_with_sight() {
        let provided = "L.......L\n.........\n.........\nL........";
        let adjacent = get_adjacent_with_sight(&generator(provided).seats, 0, 0);
        assert_eq!(2, adjacent.len());
    }

    #[test]
    fn it_solves_part_one() {
        let provided = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(37, solve_part1(&generator(provided)));
    }

    #[test]
    fn it_solves_part_two() {
        let provided = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(26, solve_part2(&generator(provided)));
    }
}
