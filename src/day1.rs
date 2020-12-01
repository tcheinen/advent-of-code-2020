use std::collections::HashSet;
use itertools::Itertools;

/// https://adventofcode.com/2020/day/1

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u32> {
    input.split("\n").flat_map(|x| x.parse()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let nums: HashSet<u32> = input.iter().cloned().collect();
    input
        .iter()
        .find(|x| nums.contains(&(2020 - **x)))
        .map(|x| x * (2020 - *x))
        .unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let nums: HashSet<u32> = input.iter().cloned().collect();
    input
        .iter()
        .sorted()
        .flat_map(|a| {
            input
                .iter()
                .filter(|x| **x < (2020 - *a))
                .map(|x| (a, x))
                .collect::<Vec<_>>()
                .into_iter()
        })
        .find(|(a, b)| nums.contains(&(2020 - **a - **b)))
        .map(|(a, b)| *a * *b * (2020 - *a - *b))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "1721
979
366
299
675
1456";
        assert_eq!(514579, solve_part1(&generator(provided)));
    }

    #[test]
    fn it_works_part2() {
        let provided = "1721
979
366
299
675
1456";
        assert_eq!(241861950, solve_part2(&generator(provided)));
    }
}
