use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_until, take_while_m_n};
use nom::combinator::{map_res, not, opt};
use nom::multi::{many0, many1};
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

/// https://adventofcode.com/2020/day/4

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, |x: char| x.is_digit(16)), |x| {
        u8::from_str_radix(x, 16)
    })(input)
}

fn hex_color(input: &str) -> IResult<&str, (u8, u8, u8)> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;
    Ok((input, (red, green, blue)))
}

fn parse_pair(input: &str) -> IResult<&str, (String, String)> {
    let (input, _) = not(tag("\n"))(input)?;
    let (input, first) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, second) = take_till(|x| x == ' ' || x == '\n')(input)?;
    let (input, _) = opt(alt((tag(" "), tag("\n"))))(input)?;
    Ok((input, (first.to_string(), second.to_string())))
}

fn parse_passport(input: &str) -> IResult<&str, HashMap<String, String>> {
    let (input, pairs) = many1(parse_pair)(input)?;
    let (input, _) = opt(tag("\n"))(input)?;
    Ok((input, pairs.into_iter().collect()))
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<HashMap<String, String>> {
    let (_, passports) = many0(parse_passport)(input).unwrap();
    passports
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[HashMap<String, String>]) -> usize {
    input
        .iter()
        .filter(|x| {
            x.contains_key("byr")
                && x.contains_key("iyr")
                && x.contains_key("eyr")
                && x.contains_key("hgt")
                && x.contains_key("hcl")
                && x.contains_key("ecl")
                && x.contains_key("pid")
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[HashMap<String, String>]) -> usize {
    input
        .iter()
        .filter(|x| {
            x.contains_key("byr")
                && x.contains_key("iyr")
                && x.contains_key("eyr")
                && x.contains_key("hgt")
                && x.contains_key("hcl")
                && x.contains_key("ecl")
                && x.contains_key("pid")
        })
        .filter(|x| (1920..=2002).contains(&x.get("byr").unwrap().parse::<i32>().unwrap()))
        .filter(|x| (2010..=2020).contains(&x.get("iyr").unwrap().parse::<i32>().unwrap()))
        .filter(|x| (2020..=2030).contains(&x.get("eyr").unwrap().parse::<i32>().unwrap()))
        .filter(|x| {
            let height = x.get("hgt").unwrap();
            if height.ends_with("cm") {
                (150..=193).contains(&height.trim_end_matches("cm").parse::<i32>().unwrap())
            } else if height.ends_with("in") {
                (59..=76).contains(&height.trim_end_matches("in").parse::<i32>().unwrap())
            } else {
                false
            }
        })
        .filter(|x| hex_color(x.get("hcl").unwrap()).is_ok())
        .filter(|x| match x.get("ecl").unwrap().as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        })
        .filter(|x| {
            let num = format!("{:09}", x.get("pid").unwrap());
            num.len() == 9 && num.chars().all(|x| x.is_digit(10))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    #[test]
    fn it_parses_pair() {
        let provided = "ecl:gry ";
        assert_eq!(
            ("ecl".to_string(), "gry".to_string()),
            parse_pair(provided).ok().unwrap().1
        );
        let provided = "ecl:gry\n";
        assert_eq!(
            ("ecl".to_string(), "gry".to_string()),
            parse_pair(provided).ok().unwrap().1
        );
        let provided = "ecl:gry\n";
        assert_eq!(
            ("ecl".to_string(), "gry".to_string()),
            parse_pair(provided).ok().unwrap().1
        );
    }

    #[test]
    fn it_parses_passport() {
        let provided = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm\n\n";
        assert_eq!(
            vec![
                ("ecl".to_string(), "gry".to_string()),
                ("pid".to_string(), "860033327".to_string()),
                ("eyr".to_string(), "2020".to_string()),
                ("hcl".to_string(), "#fffffd".to_string()),
                ("byr".to_string(), "1937".to_string()),
                ("iyr".to_string(), "2017".to_string()),
                ("cid".to_string(), "147".to_string()),
                ("hgt".to_string(), "183cm".to_string())
            ]
            .into_iter()
            .sorted()
            .collect_vec(),
            parse_passport(provided)
                .ok()
                .unwrap()
                .1
                .into_iter()
                .sorted()
                .collect_vec()
        );

        let provided = "hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in\n";

        assert_eq!(
            vec![
                ("hcl".to_string(), "#cfa07d".to_string()),
                ("eyr".to_string(), "2025".to_string()),
                ("pid".to_string(), "166559648".to_string()),
                ("iyr".to_string(), "2011".to_string()),
                ("ecl".to_string(), "brn".to_string()),
                ("hgt".to_string(), "59in".to_string()),
            ]
            .into_iter()
            .sorted()
            .collect_vec(),
            parse_passport(provided)
                .ok()
                .unwrap()
                .1
                .into_iter()
                .sorted()
                .collect_vec()
        );

        let provided = "cid:244
hcl:#866857 ecl:amb byr:1931
eyr:1928 pid:557376401 hgt:182cm iyr:2013
";

        assert_eq!(
            vec![
                ("ecl".to_string(), "amb".to_string()),
                ("hcl".to_string(), "#866857".to_string()),
                ("byr".to_string(), "1931".to_string()),
                ("cid".to_string(), "244".to_string()),
                ("eyr".to_string(), "1928".to_string()),
                ("pid".to_string(), "557376401".to_string()),
                ("hgt".to_string(), "182cm".to_string()),
                ("iyr".to_string(), "2013".to_string()),
            ]
            .into_iter()
            .sorted()
            .collect_vec(),
            parse_passport(provided)
                .ok()
                .unwrap()
                .1
                .into_iter()
                .sorted()
                .collect_vec()
        );
    }

    #[test]
    fn it_works_part1() {
        let provided = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in\n";
        assert_eq!(2, solve_part1(&generator(provided)));
    }

    #[test]
    fn it_works_part1_input() {
        let provided = include_str!("../input/2020/day4.txt");
        assert_eq!(196, solve_part1(&generator(provided)));
    }

    #[test]
    fn it_works_part2() {
        let provided = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in\n";
        assert_eq!(2, solve_part2(&generator(provided)));
    }
}
