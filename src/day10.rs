use std::collections::HashMap;
use itertools::Itertools;
use cached::UnboundCache;
use cached::proc_macro::cached;

/// https://adventofcode.com/2020/day/10

#[aoc_generator(day10)]
pub fn generator(input: &str) -> HashMap<usize, Vec<usize>> {
    let mut nums = input
        .lines()
        .filter_map(|x| x.parse::<usize>().ok())
        .sorted()
        .collect_vec();
    nums.push(nums.last().unwrap() + 3);
    nums.push(0);
    nums.push(0);
    nums.push(0);
    nums.insert(0, 0);

    nums
        .windows(4)
        .map(|x| (x[0], x.into_iter().filter(|y| **y <= x[0] + 3 && **y != x[0]).cloned().collect_vec()))
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &HashMap<usize, Vec<usize>>) -> usize {
    let start = input.keys().min().unwrap().clone();
    let end = input.keys().max().unwrap().clone();
    let path = pathfinding::directed::dfs::dfs(start, |x| input.get(x).unwrap().clone(), |x| *x == end).unwrap();
    path.windows(2).filter(|x| x[1] - x[0] == 1).count() * path.windows(2).filter(|x| x[1] - x[0] == 3).count()
}

#[cached(
type = "UnboundCache<usize, usize>",
create = "{ UnboundCache::new() }",
convert = r#"{ node << 16 | target}"#
)]
fn total_paths(input: &HashMap<usize, Vec<usize>>, node: usize, target: usize) -> usize {
    if node == target {
        1
    } else {
        input.get(&node).unwrap().iter().filter(|x| **x != 0).map(|x| total_paths(input, *x, target)).sum()
    }
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &HashMap<usize, Vec<usize>>) -> usize {
    let start = input.keys().min().unwrap().clone();
    let end = input.keys().max().unwrap().clone();
    total_paths(input, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates() {
        let provided = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

        println!("{:#?}", generator(provided));
    }

    #[test]
    fn it_solves_part1() {
        let provided = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(35, solve_part1(&generator(provided)))
    }

}
