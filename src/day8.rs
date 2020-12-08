use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::multispace0;
use nom::combinator::opt;
use std::collections::HashSet;
use nom::multi::many1;

use nom::IResult;

/// https://adventofcode.com/2020/day/7

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Operation {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Operation {
    pub fn is_acc(&self) -> bool {
        match self {
            Operation::Acc(_) => true,
            _ => false,
        }
    }
    pub fn flip(&self) -> Self {
        match self {
            Operation::Nop(x) => Operation::Jmp(*x),
            Operation::Jmp(x) => Operation::Nop(*x),
            Operation::Acc(x) => Operation::Acc(*x),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ExecutionReport {
    halting: bool,
    acc: isize,
    visited: HashSet<usize>,
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, op) = take_until(" ")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, param) = take_while(|x: char| x == '-' || x == '+' || x.is_digit(10))(input)?;
    let (input, _) = opt(tag("\n"))(input)?;
    let param = param.parse::<isize>().unwrap();
    match op {
        "nop" => Ok((input, Operation::Nop(param))),
        "acc" => Ok((input, Operation::Acc(param))),
        "jmp" => Ok((input, Operation::Jmp(param))),
        _ => unreachable!(),
    }
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Operation> {
    many1(operation)(input).unwrap().1
}

fn execute_from(input: &[Operation], base: usize) -> ExecutionReport {
    let mut rip = base;
    let mut acc = 0;
    let mut visited: HashSet<usize> = HashSet::new();
    while rip < input.len() {
        if !visited.insert(rip) {
            return ExecutionReport {
                acc: acc,
                halting: true,
                visited,
            };
        }
        match input[rip] {
            Operation::Nop(_) => {}
            Operation::Acc(param) => acc += param,
            Operation::Jmp(param) => {
                if param.is_positive() {
                    rip += param as usize
                } else {
                    rip -= param.abs() as usize
                }
                continue;
            }
        }

        rip += 1;
    }
    ExecutionReport {
        acc,
        halting: false,
        visited,
    }
}

fn find_halting(input: &[Operation]) -> HashSet<usize> {
    let mut unvisited: HashSet<usize> = (0..input.len())
        .into_iter()
        .filter(|x| !input[*x].is_acc())
        .collect();
    let mut halting: HashSet<usize> = HashSet::new();
    while let Some(next) = unvisited.iter().next().cloned() {
        unvisited.remove(&next);
        let report = execute_from(input, next);
        if report.halting {
            halting = halting.union(&report.visited).cloned().collect()
        }
    }
    halting
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Operation]) -> isize {
    execute_from(input, 0).acc
}
#[aoc(day8, part2)]
pub fn solve_part2(input: &[Operation]) -> isize {
    let halting = find_halting(input);
    let (fixed_program, _) = input
        .iter()
        .map(|x| x.flip())
        .enumerate()
        .filter(|(index, _)| halting.contains(index))
        .map(|(index, x)| {
            let mut fixed_program = input.to_vec();
            fixed_program[index] = x;
            let report = execute_from(&fixed_program, 0);
            (fixed_program, report)
        })
        .find(|(_, x)| !x.halting)
        .unwrap();
    execute_from(&fixed_program, 0).acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates() {
        let provided = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(9, generator(provided).len());
    }

    #[test]
    fn it_solves_part1_mini() {
        let provided = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(5, solve_part1(&generator(provided)));
    }

    #[test]
    fn it_solves_part2_mini() {
        let provided = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(8, solve_part2(&generator(provided)));
    }

    #[test]
    fn it_solves_part2() {
        let provided = include_str!("../input/2020/day8.txt");
        assert_eq!(1245, solve_part2(&generator(provided)));
    }
}
