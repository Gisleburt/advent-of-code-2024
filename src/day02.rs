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

    fn part2(input: &str) -> String {

        let count = input.lines()
            .map(|line| get_list(line).expect("Could not parse input"))
            .filter(|list| is_loosely_safely_ascending(list) || is_loosely_safely_descending(list))
            .count();
        count.to_string()
    }
}

fn safe_ascent(left: usize, right: usize) -> bool {
    right > left && right - left <= 3
}

fn safe_descent(left: usize, right: usize) -> bool {
    left > right && left - right <= 3
}

fn is_safely_ascending(list: &[usize]) -> bool {
    list.windows(2).all(|w| safe_ascent(w[0] , w[1]))
}

fn is_safely_descending(list: &[usize]) -> bool {
    list.windows(2).all(|w| safe_descent(w[0] , w[1]))
}


fn is_loosely_safely_ascending(list: &[usize]) -> bool {
    // Not very memory efficient but 🤷🏻‍♂️
    let mut new_list_left: Vec<_> = list.iter().copied().collect();
    let mut new_list_right: Vec<_> = list.iter().copied().collect();

    // remove first bad value
    if let Some(i) = list.windows(2).position(|w| !safe_ascent(w[0], w[1])) {
        new_list_left.remove(i);
        new_list_right.remove(i + 1usize);
    }

    is_safely_ascending(&new_list_left) || is_safely_ascending(&new_list_right)
}

fn is_loosely_safely_descending(list: &[usize]) -> bool {
    let mut new_list_left: Vec<_> = list.iter().copied().collect();
    let mut new_list_right: Vec<_> = list.iter().copied().collect();

    // remove first bad value
    if let Some(i) = list.windows(2).position(|w| !safe_descent(w[0], w[1])) {
        new_list_left.remove(i);
        new_list_right.remove(i + 1usize);
    }

    is_safely_descending(&new_list_left) || is_safely_descending(&new_list_right)
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

    #[test]
    fn test_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!(AdventPuzzle::part2(input), "4");
    }
}
