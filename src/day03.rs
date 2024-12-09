use crate::solution::Solution;
use nom::character::complete::{anychar, digit1};
use nom::combinator::map_res;
use nom::multi::{many1, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::{IResult, Parser};
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;

pub struct AdventPuzzle;

impl Solution for AdventPuzzle {
    fn part1(input: &str) -> String {
        input
            .lines()
            .map(|line| run_line_basic(line))
            .sum::<usize>()
            .to_string()
    }

    fn part2(input: &str) -> String {
        let instructions: Vec<_> = input
            .lines()
            .map(|line| parse_line_do_dont(line).expect("Could not parse line"))
            .map(|(_, instructions)| instructions)
            .collect();

        let combined_instructions = Instructions::combine(&instructions);
        combined_instructions.process().to_string()
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Instruction {
    Multiply(usize, usize),
    Do,
    Dont,
}

#[derive(PartialEq, Debug)]
struct Instructions(Vec<Instruction>);

impl Instructions {
    fn process(&self) -> usize {
        let mut do_multiply = true;
        let mut total = 0;
        for instruction in &self.0 {
            match instruction {
                Instruction::Multiply(a, b) => {
                    if do_multiply {
                        total += a * b;
                    }
                }
                Instruction::Do => do_multiply = true,
                Instruction::Dont => do_multiply = false,
            }
        }
        total
    }

    fn combine(instructions: &[Instructions]) -> Instructions {
        let new_instructions = instructions
            .into_iter()
            .flat_map(|i| i.0.iter().copied())
            .collect::<Vec<_>>();
        Instructions(new_instructions)
    }
}

fn parse_mul(input: &str) -> IResult<&str, Instruction, ErrorTree<&str>> {
    delimited(
        tag("("),
        separated_pair(
            map_res(digit1, str::parse),
            tag(","),
            map_res(digit1, str::parse),
        ),
        tag(")"),
    )
    .map(|(a, b)| Instruction::Multiply(a, b))
    .preceded_by(tag("mul"))
    .parse(input)
}

fn parse_do(input: &str) -> IResult<&str, Instruction, ErrorTree<&str>> {
    tag("do()").map(|_| Instruction::Do).parse(input)
}

fn parse_dont(input: &str) -> IResult<&str, Instruction, ErrorTree<&str>> {
    tag("don't()").map(|_| Instruction::Dont).parse(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction, ErrorTree<&str>> {
    parse_mul.or(parse_do).or(parse_dont).parse(input)
}

fn parse_line_basic(input: &str) -> IResult<&str, Instructions, ErrorTree<&str>> {
    many1(many_till(anychar, parse_mul))
        .parse(input)
        .map(|(_, result)| {
            let instructions = Instructions(result.into_iter().map(|(_s, m)| m).collect());
            ("", instructions) // Dump the remainder
        })
}

fn run_line_basic(input: &str) -> usize {
    let (_, list) = parse_line_basic(input).expect("failed to parse line");
    list.process()
}

fn parse_line_do_dont(input: &str) -> IResult<&str, Instructions, ErrorTree<&str>> {
    many1(many_till(anychar, parse_instruction))
        .parse(input)
        .map(|(_, result)| {
            let instructions = Instructions(result.into_iter().map(|(_s, m)| m).collect());
            ("", instructions) // Dump the remainder
        })
}

fn run_line_do_dont(input: &str) -> usize {
    let (_, list) = parse_line_do_dont(input).expect("failed to parse line");
    list.process()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_mul() {
        let (_, pair) = parse_mul("mul(1,2)").expect("failed to parse mul");
        assert_eq!(pair, Instruction::Multiply(1, 2));
        assert!(parse_mul("mul(1, 2)").is_err())
    }

    #[test]
    fn test_parse_line_basic() {
        let (_, pairs) = parse_line_basic(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        )
        .expect("failed to parse mul");
        assert_eq!(
            pairs,
            Instructions(vec![
                Instruction::Multiply(2, 4),
                Instruction::Multiply(5, 5),
                Instruction::Multiply(11, 8),
                Instruction::Multiply(8, 5)
            ])
        );
    }

    #[test]
    fn test_parse_line_do_dont() {
        let (_, pairs) = parse_line_do_dont(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        )
        .expect("failed to parse mul");
        assert_eq!(
            pairs,
            Instructions(vec![
                Instruction::Multiply(2, 4),
                Instruction::Dont,
                Instruction::Multiply(5, 5),
                Instruction::Multiply(11, 8),
                Instruction::Do,
                Instruction::Multiply(8, 5)
            ])
        );
    }

    #[test]
    fn test_part2_multi_line() {
        let input = "don't()
mul(5,5)do()mul(3,3)";
        assert_eq!(AdventPuzzle::part2(input), "9");
    }

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(AdventPuzzle::part1(input), "161");
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(AdventPuzzle::part2(input), "48");
    }
}
