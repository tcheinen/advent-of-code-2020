use itertools::Itertools;
use nom::bytes::complete::{tag, take_until, take_while1};
use nom::character::complete::multispace0;
use nom::combinator::{opt};
use nom::lib::std::collections::VecDeque;
use nom::multi::{many1};

use nom::IResult;

use std::collections::{HashMap};

/// https://adventofcode.com/2020/day/7

#[derive(Debug, Eq, PartialEq)]
pub struct Constraint {
    source: String,
    contains: Vec<(u32, String)>,
}

fn bag_constraint(input: &str) -> IResult<&str, Constraint> {
    fn is_digit(c: char) -> bool {
        nom::character::is_digit(c as u8)
    }
    fn parse_contain(input: &str) -> IResult<&str, Option<(u32, String)>> {
        let (input, none) = opt(tag("no other bags"))(input)?;
        if none.is_some() {
            return Ok((input, None));
        }
        let (input, amount) = take_while1(is_digit)(input)?;
        let (input, _) = multispace0(input)?;
        let (input, source) = take_until(" bag")(input)?;
        let (input, _) = tag(" bag")(input)?;
        let (input, _) = opt(tag("s"))(input)?;
        let (input, _) = opt(tag(", "))(input)?;
        Ok((
            input,
            Some((amount.parse::<u32>().unwrap(), source.to_string())),
        ))
    }

    let (input, source) = take_until(" bags")(input)?;
    let (input, _) = tag(" bags contain ")(input)?;
    let (input, contains) = many1(parse_contain)(input)?;
    let (input, _) = opt(tag(".\n"))(input)?;
    Ok((
        input,
        Constraint {
            source: source.to_string(),
            contains: contains.into_iter().flat_map(|x| x).collect(),
        },
    ))
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> HashMap<String, Vec<(u32, String)>> {
    many1(bag_constraint)(input)
        .unwrap()
        .1
        .iter()
        .map(|x| (x.source.clone(), x.contains.clone()))
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &HashMap<String, Vec<(u32, String)>>) -> usize {
    let dt = input
        .into_iter()
        .flat_map(|(k, v)| v.iter().map(|x| (k.clone(), x.1.clone())).collect_vec())
        .map(|(a, b)| (b, a)) // reverse for djikstra
        .collect::<Vec<_>>();

    pathfinding::directed::dijkstra::dijkstra_all(&"shiny gold".to_string(), |x| {
        dt.iter()
            .filter(|(k, _v)| k == x)
            .map(|(_, v)| (v.clone(), 1))
            .collect_vec()
    })
    .len()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &HashMap<String, Vec<(u32, String)>>) -> usize {
    let mut queue: VecDeque<String> = VecDeque::from(vec!["shiny gold".to_string()]);
    let mut sum = 0;
    while let Some(next) = queue.pop_front() {
        if let Some(children) = input.get(&next) {
            for (num, child) in children {
                sum += num;
                for _ in 0..*num {
                    queue.push_back(child.clone());
                }
            }
        }
    }
    sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_one_line() {
        let provided = "light red bags contain 1 bright white bag, 2 muted yellow bags.\
        ";
        assert_eq!(
            Constraint {
                source: "light red".to_string(),
                contains: vec![
                    (1, "bright white".to_string()),
                    (2, "muted yellow".to_string())
                ]
            },
            bag_constraint(provided).unwrap().1
        );
    }

    #[test]
    fn it_generates() {
        let provided = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(9, generator(provided).len());
    }

    #[test]
    fn it_solves_part1_mini() {
        let provided = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(4, solve_part1(&generator(provided)));
    }

    #[test]
    fn it_solves_part2_mini() {
        let provided = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(126, solve_part2(&generator(provided)));
    }
}
