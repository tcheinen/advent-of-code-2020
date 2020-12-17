use std::collections::HashSet;

/// https://adventofcode.com/2020/day/9

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|x| x.parse::<usize>().ok())
        .collect()
}

pub fn two_sum(input: &[usize], sum: usize) -> Option<(usize, usize)> {
    input
        .iter()
        .zip(std::iter::repeat(
            &input.iter().cloned().collect::<HashSet<usize>>(),
        ))
        .filter(|(x, _)| **x < sum)
        .find(|(x, nums)| {
            nums.get(&(sum - **x)).is_some() && nums.get(&(sum - **x)).map(|y| **x != *y).unwrap()
        })
        .map(|(x, _)| (*x, (sum - *x)))
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    input
        .windows(26)
        .map(|x| (x[..25].to_vec(), x[25]))
        .find(|(a, b)| two_sum(&a.clone(), *b).is_none())
        .map(|(_, b)| b)
        .unwrap()
}

pub fn search_window(input: &[usize], length: usize, sum: usize) -> Option<Vec<usize>> {
    input
        .windows(length)
        .find(|x| x.iter().sum::<usize>() == sum)
        .map(|x| x.to_vec())
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let sum = solve_part1(input);
    let solution = (2..input.len())
        .filter_map(|x| search_window(input, x, sum))
        .next()
        .unwrap();
    solution.iter().min().unwrap() + solution.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Some((15, 25)), two_sum(&[35, 20, 15, 25, 47], 40))
    }

    #[test]
    fn it_generates() {
        let provided = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(20, generator(provided).len());
    }

    #[test]
    fn it_solves_part1() {
        let provided = include_str!("../input/2020/day9.txt");
        assert_eq!(21806024, solve_part1(&generator(provided)));
    }


    #[test]
    fn it_solves_part2() {
        let provided = include_str!("../input/2020/day9.txt");
        assert_eq!(2986195, solve_part2(&generator(provided)));
    }
}
