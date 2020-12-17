use itertools::Itertools;

/// https://adventofcode.com/2020/day/13

#[aoc_generator(day13)]
pub fn generator(input: &str) -> (isize, Vec<Option<isize>>) {
    (
        input.lines().nth(0).unwrap().parse::<isize>().unwrap(),
        input
            .lines()
            .nth(1)
            .unwrap()
            .split(",")
            .map(|x| x.parse::<isize>().ok())
            .collect_vec(),
    )
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &(isize, Vec<Option<isize>>)) -> isize {
    let (timestamp, buses) = input;
    let (first_timestamp, bus) = (*timestamp..isize::max_value())
        .filter_map(|x| {
            Some((
                x,
                buses
                    .iter()
                    .filter_map(|y| *y)
                    .filter(|y| x % y == 0)
                    .next()?,
            ))
        })
        .next()
        .unwrap();
    (first_timestamp - timestamp) * bus
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &(isize, Vec<Option<isize>>)) -> isize {
    let (_, buses) = input;
    buses
        .iter()
        .enumerate()
        .fold((0isize, 1isize), |(solution, step), (offset, bus_id)| {
            if let Some(bus_id) = bus_id {
                ((solution..isize::MAX)
                     .step_by(step as usize)
                     .find(|x| (x + offset as isize) % bus_id == 0)
                     .unwrap(), step * bus_id)
            } else {
                (solution, step)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part_one() {
        let provided = "939
7,13,x,x,59,x,31,19";

        assert_eq!(295, solve_part1(&generator(provided)));
    }

    #[test]
    fn it_solves_part_two() {
        let provided = "939
7,13,x,x,59,x,31,19";

        assert_eq!(1068781, solve_part2(&generator(provided)));
    }
}
