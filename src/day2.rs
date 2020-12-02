use itertools::Itertools;
use std::collections::HashSet;

/// https://adventofcode.com/2020/day/2

#[derive(Clone)]
pub struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<(Policy, String)> {
    input
        .split("\n")
        .filter_map(|x| x.split(": ").collect_tuple::<(&str, &str)>())
        .filter_map(|(a, b)| Some((a.split(" ").collect_tuple::<(&str, &str)>()?, b)))
        .filter_map(|((range, letter), password)| {
            Some((
                range.split("-").collect_tuple::<(&str, &str)>()?,
                letter,
                password,
            ))
        })
        .filter_map(|((min, max), letter, password)| {
            Some((
                Policy {
                    min: min.parse().ok()?,
                    max: max.parse().ok()?,
                    letter: letter.chars().next()?,
                },
                password.to_string(),
            ))
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(Policy, String)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| {
            (policy.min..=policy.max)
                .contains(&password.chars().filter(|x| *x == policy.letter).count())
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(Policy, String)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| {
            (password.chars().nth(policy.min - 1).unwrap_or('\x00') == policy.letter)
                != (password.chars().nth(policy.max - 1).unwrap_or('\x00') == policy.letter)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(2, solve_part1(&generator(provided)));
    }

    #[test]
    fn it_works_part2() {
        let provided = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(1, solve_part2(&generator(provided)));
    }
}
