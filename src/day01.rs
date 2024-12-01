use crate::solution::Solution;
use nom::character::complete::{digit1, line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};
use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::final_parser;
use nom_supreme::ParserExt;

pub struct AdventPuzzle;

impl Solution for AdventPuzzle {
    fn part1(input: &str) -> String {
        let (mut left, mut right) = get_list(input).expect("Advent Puzzle parsing failed");
        left.sort();
        right.sort();

        let total: u64 = left
            .into_iter()
            .zip(right)
            .map(|(left, right)| left.abs_diff(right))
            .sum();

        total.to_string()
    }

    fn part2(input: &str) -> String {
        let (left, right) = get_list(input).expect("Advent Puzzle parsing failed");

        let total: usize = left
            .iter()
            .map(|l| right.iter().filter(|r| *r == l).count() * *l as usize)
            .sum();

        total.to_string()
    }
}

fn parse_pair(input: &str) -> IResult<&str, (u64, u64), ErrorTree<&str>> {
    separated_pair(
        digit1.map_res(str::parse).context("Should be a number"),
        space1.context("Pairs of numbers should be separated by spaces"),
        digit1.map_res(str::parse).context("Should be a number"),
    )
    .parse(input)
}

fn parse_list(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>), ErrorTree<&str>> {
    separated_list1(line_ending, parse_pair)
        .map(|list| list.into_iter().unzip())
        .parse(input)
}

fn get_list(input: &str) -> Result<(Vec<u64>, Vec<u64>), ErrorTree<&str>> {
    final_parser(parse_list.terminated(line_ending.opt()).all_consuming())(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(AdventPuzzle::part1(input), "11");
    }

    #[test]
    fn test_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(AdventPuzzle::part2(input), "31");
    }
}
