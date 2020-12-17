use itertools::Itertools;
use nom::lib::std::convert::TryFrom;
use pathfinding::num_traits::FloatConst;
use std::convert::TryInto;

/// https://adventofcode.com/2020/day/12

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    North,
    West,
    East,
    South,
    Left,
    Right,
    Forward,
}

impl TryFrom<&str> for Action {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "N" => Ok(Action::North),
            "W" => Ok(Action::West),
            "E" => Ok(Action::East),
            "S" => Ok(Action::South),
            "L" => Ok(Action::Left),
            "R" => Ok(Action::Right),
            "F" => Ok(Action::Forward),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Instruction {
    pub action: Action,
    pub value: isize,
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|line| {
            Some(Instruction {
                action: line[0..1].try_into().unwrap(),
                value: line[1..].parse::<isize>().unwrap(),
            })
        })
        .collect_vec()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Instruction]) -> isize {
    let (x, y, angle) =
        input
            .iter()
            .fold((0isize, 0isize, 0isize), |(x, y, angle), instruction| {
                match instruction.action {
                    Action::North => (x, y + instruction.value, angle),
                    Action::South => (x, y - instruction.value, angle),
                    Action::East => (x + instruction.value, y, angle),
                    Action::West => (x - instruction.value, y, angle),
                    Action::Left => (x, y, (angle - instruction.value).rem_euclid(360)),
                    Action::Right => (x, y, (angle + instruction.value).rem_euclid(360)),
                    Action::Forward => match angle {
                        0 => (x + instruction.value, y, angle),
                        90 => (x, y - instruction.value, angle),
                        180 => (x - instruction.value, y, angle),
                        270 => (x, y + instruction.value, angle),
                        x => panic!("logic bug vis a vis angles, {}", x),
                    },
                }
            });
    x.abs() + y.abs()
}

fn rotate(
    (pivot_x, pivot_y): (isize, isize),
    (point_x, point_y): (isize, isize),
    angle: f64,
) -> (isize, isize) {
    let angle = angle * (f64::PI() / 180.0);
    let x = (point_x - pivot_x) as f64;
    let y = (point_y - pivot_y) as f64;
    let xn = x * angle.cos() + y * angle.sin();
    let yn = y * angle.cos() - x * angle.sin();
    (xn.round() as isize + pivot_x, yn.round() as isize + pivot_y)
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Instruction]) -> isize {
    let ((x, y), _) = input.iter().fold(
        ((0isize, 0isize), (10isize, 1isize)),
        |((x, y), (wx, wy)), instruction| {
            match instruction.action {
                Action::North => ((x, y), (wx, wy + instruction.value)),
                Action::South => ((x, y), (wx, wy - instruction.value)),
                Action::East => ((x, y), (wx + instruction.value, wy)),
                Action::West => ((x, y), (wx - instruction.value, wy)),
                Action::Left => ((x, y), rotate((0, 0), (wx, wy), -1.0 * instruction.value as f64)),
                Action::Right => (
                    (x, y),
                    rotate((0, 0), (wx, wy), instruction.value as f64),
                ),
                Action::Forward => {
                    let offx = (wx) * instruction.value;
                    let offy = (wy) * instruction.value;
                    ((x + offx, y + offy), (wx, wy))
                }
            }
        },
    );
    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates() {
        let provided = "S1\nL2";
        assert_eq!(
            vec![
                Instruction {
                    action: Action::South,
                    value: 1
                },
                Instruction {
                    action: Action::Left,
                    value: 2
                }
            ],
            generator(provided)
        );
    }

    #[test]
    fn it_rotates() {
        assert_eq!((4, -10), rotate((0, 0), (10, 4), 90.0));
        assert_eq!((-4, 10), rotate((0, 0), (10, 4), -90.0));
    }

    #[test]
    fn it_solves_part_one() {
        let provided = "F10
N3
F7
R90
F11";
        assert_eq!(25, solve_part1(&generator(provided)));
    }

    #[test]
    fn it_solves_part_two() {
        let provided = "F10
N3
F7
R90
F11";
        assert_eq!(286, solve_part2(&generator(provided)));
    }
}
