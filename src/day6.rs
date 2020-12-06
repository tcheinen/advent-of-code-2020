use itertools::Itertools;
use reduce::Reduce;
use std::collections::HashSet;

/// https://adventofcode.com/2020/day/6

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Group {
    pub people: Vec<Vec<char>>,
}

impl Group {
    pub fn count_unique(&self) -> usize {
        self.people.iter().flat_map(|x| x.iter()).unique().count()
    }
    pub fn count_intersection(&self) -> usize {
        self.people
            .iter()
            .map(|x| x.into_iter().collect::<HashSet<_>>())
            .reduce(|a, b| a.intersection(&b).cloned().collect())
            .map(|x| x.len())
            .unwrap_or(0)
    }
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Group> {
    input
        .split("\n\n")
        .map(|x| Group {
            people: x.lines().map(|y| y.chars().collect_vec()).collect_vec(),
        })
        .collect_vec()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Group]) -> usize {
    input.iter().map(|x| x.count_unique()).sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Group]) -> usize {
    input.iter().map(|x| x.count_intersection()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates() {
        let provided = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(5, generator(provided).iter().count());
    }

    #[test]
    fn it_solves_part1_mini() {
        let provided = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(11, solve_part1(&generator(provided)));
    }

    #[test]
    fn it_solves_part2_mini() {
        let provided = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(6, solve_part2(&generator(provided)));
    }
}
