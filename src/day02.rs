use nom::character::complete::{digit1, space1};
use nom::IResult;
use nom::multi::separated_list1;
use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::final_parser;
use nom_supreme::ParserExt;
use crate::solution::Solution;

pub struct AdventPuzzle;

impl Solution for AdventPuzzle {
    fn part1(input: &str) -> String {
        let count = input.lines()
            .map(|line| get_list(line).expect("Could not parse input"))
            .filter(|list| is_safely_ascending(list) || is_safely_descending(list))
            .count();
        count.to_string()
    }

    fn part2(_input: &str) -> String {
        todo!()
    }
}

fn is_safely_ascending(list: &[usize]) -> bool {
    list.windows(2).all(|w| {
        let left = w[0];
        let right = w[1];
        right > left && right - left <= 3
    })
}

fn is_safely_descending(list: &[usize]) -> bool {
    list.windows(2).all(|w| {
        let left = w[0];
        let right = w[1];
        right < left && left - right <= 3
    })
}

fn parse_list(input: &str) -> IResult<&str, Vec<usize>, ErrorTree<&str>> {
    separated_list1(space1, digit1.map_res(str::parse))(input)
}

fn get_list(input: &str) -> Result<Vec<usize>, ErrorTree<&str>> {
    final_parser(parse_list.all_consuming())(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!(AdventPuzzle::part1(input), "2");
    }

    #[ignore]
    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(AdventPuzzle::part2(input), "");
    }
}
