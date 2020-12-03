use itertools::Itertools;
use std::collections::HashSet;

/// https://adventofcode.com/2020/day/3

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<bool>> {
    input
        .split("\n")
        .map(|line| line.chars().map(|spot| spot == '#').collect_vec())
        .collect_vec()
}

fn slope(input: &[Vec<bool>], right: usize, down: usize) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(i, line)| i % down == 0)
        .map(|(i, line)| {
            if line[(i as f32 * (right as f32 / down as f32)) as usize % line.len()] {
                1
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<bool>]) -> usize {
    slope(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<bool>]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1usize, |sum, (right, down)| {
            sum * slope(input, *right, *down)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        assert_eq!(7, solve_part1(&generator(provided)));
    }

    #[test]
    fn it_works_part2() {
        let provided = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        assert_eq!(336, solve_part2(&generator(provided)));
    }
}
