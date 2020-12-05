use itertools::Itertools;

/// https://adventofcode.com/2020/day/5

pub fn parse_seat(input: &str) -> (u8, u8) {
    (
        input
            .chars()
            .take(7)
            .map(|x| (x == 'B') as u8)
            .enumerate()
            .fold(0u8, |acc, (index, val)| acc | (val << (6 - index))),
        input
            .chars()
            .skip(7)
            .map(|x| (x == 'R') as u8)
            .enumerate()
            .fold(0u8, |acc, (index, val)| acc | (val << (2 - index))),
    )
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<(u8, u8)> {
    input.lines().map(parse_seat).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[(u8, u8)]) -> usize {
    input
        .into_iter()
        .map(|(a, b)| (*a as usize, *b as usize))
        .map(|(a, b)| a * 8 + b)
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[(u8, u8)]) -> usize {
    input
        .iter()
        .map(|(a, b)| (*a as usize, *b as usize))
        .map(|(a, b)| a * 8 + b)
        .sorted()
        .collect_vec()
        .windows(2)
        .map(|x| (x[0], x[1]))
        .find(|(a, b)| *a + 1 != *b)
        .unwrap()
        .0
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_seats() {
        assert_eq!((44, 5), parse_seat("FBFBBFFRLR"));
        assert_eq!((70, 7), parse_seat("BFFFBBFRRR"));
        assert_eq!((14, 7), parse_seat("FFFBBBFRRR"));
        assert_eq!((102, 4), parse_seat("BBFFBBFRLL"));
    }

    #[test]
    fn it_works_part1() {
        assert_eq!(
            892,
            solve_part1(&generator(include_str!("../input/2020/day5.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        assert_eq!(
            625,
            solve_part2(&generator(include_str!("../input/2020/day5.txt")))
        );
    }
}
