use crate::solution::Solution;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};
use nom_supreme::ParserExt;

pub struct AdventPuzzle;

impl Solution for AdventPuzzle {
    fn part1(input: &str) -> String {
        let (mut left, mut right) = parse_list(input).expect("Advent Puzzle parsing failed").1;
        left.sort();
        right.sort();

        let total: u64 = left
            .into_iter()
            .zip(right.into_iter())
            .map(|(left, right)| left.abs_diff(right))
            .sum();

        total.to_string()
    }

    fn part2(_input: &str) -> String {
        todo!()
    }
}

fn parse_pair(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(
        map_res(digit1, str::parse),
        space1,
        map_res(digit1, str::parse),
    )(input)
}

fn parse_list(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    separated_list1(line_ending, parse_pair)(input)
        .map(|(remain, list)| (remain, list.into_iter().unzip()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
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

    #[ignore]
    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(AdventPuzzle::part2(input), "");
    }
}
