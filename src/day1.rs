use itertools::Itertools;

/// https://adventofcode.com/2020/day/1


#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u32> {
    input.split("\n").flat_map(|x| x.parse()).collect()
}

/// We're solving this the lazy way!  Pick every pair (a,b) and find the first one that sums to 2020
/// We can optimize this significantly by sorting it in ascending order first
/// Our pairs are weighted towards smaller (a,b) because obviously a pair where both are greater than 1010 can't sum to 2020
#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input
        .iter()
        .sorted()
        .combinations(2)
        .map(|x| (*x[0], *x[1]))
        .find(|(a, b)| a + b == 2020)
        .map(|(a, b)| a * b)
        .unwrap()
}

/// We're solving this the lazy way!  Pick every pair (a,b,c) and find the first one that sums to 2020
/// We can optimize this significantly by sorting it in ascending order first
/// Our pairs are weighted towards smaller (a,b,c) because obviously a pair where all are greater than 673 can't sum to 2020
#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    input
        .iter()
        .sorted()
        .combinations(3)
        .map(|x| (*x[0], *x[1], *x[2]))
        .find(|(a, b, c)| a + b + c== 2020)
        .map(|(a, b, c)| a * b * c)
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
