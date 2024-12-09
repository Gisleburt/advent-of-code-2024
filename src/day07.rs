use crate::solution::Solution;
use nom::character::complete::{digit1, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;

pub struct AdventPuzzle;

impl Solution for AdventPuzzle {
    fn part1(input: &str) -> String {
        input
            .lines()
            .map(|line| parse_equation(line).unwrap().1)
            .filter(|equation| equation.can_brute_force())
            .map(|equation| equation.total)
            .sum::<usize>()
            .to_string()
    }

    fn part2(input: &str) -> String {
        input
            .lines()
            .map(|line| parse_equation(line).unwrap().1)
            .filter(|equation| equation.can_brute_force_part2())
            .map(|equation| equation.total)
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Clone, PartialEq, Debug)]
struct SolutionTreeBranch {
    add: SolutionTree,
    multiply: SolutionTree,
}

#[derive(Clone, PartialEq, Debug)]
struct SolutionTree {
    total: usize,
    branches: Option<Box<SolutionTreeBranch>>,
}

impl SolutionTree {
    fn new(start: usize) -> Self {
        Self {
            total: start,
            branches: None,
        }
    }

    fn next(&mut self, next_value: usize) {
        if let Some(branches) = &mut self.branches {
            branches.add.next(next_value);
            branches.multiply.next(next_value);
        } else {
            self.branches = Some(Box::new(SolutionTreeBranch {
                add: SolutionTree::new(self.total + next_value),
                multiply: SolutionTree::new(self.total * next_value),
            }))
        }
    }

    fn could_equal(&self, total: usize) -> bool {
        if self.total == total {
            return true;
        }
        if let Some(branches) = &self.branches {
            return branches.add.could_equal(total) || branches.multiply.could_equal(total);
        }
        return false;
    }
}

#[derive(Clone, PartialEq, Debug)]
struct SolutionTreeBranchPart2 {
    add: SolutionTreePart2,
    multiply: SolutionTreePart2,
    concatenate: SolutionTreePart2,
}

#[derive(Clone, PartialEq, Debug)]
struct SolutionTreePart2 {
    total: usize,
    branches: Option<Box<SolutionTreeBranchPart2>>,
}

impl SolutionTreePart2 {
    fn new(start: usize) -> Self {
        Self {
            total: start,
            branches: None,
        }
    }

    fn next(&mut self, next_value: usize) {
        if let Some(branches) = &mut self.branches {
            branches.add.next(next_value);
            branches.multiply.next(next_value);
            branches.concatenate.next(next_value);
        } else {
            self.branches = Some(Box::new(SolutionTreeBranchPart2 {
                add: SolutionTreePart2::new(self.total + next_value),
                multiply: SolutionTreePart2::new(self.total * next_value),
                concatenate: SolutionTreePart2::new(
                    format!("{}{}", self.total, next_value).parse().unwrap(),
                ),
            }))
        }
    }

    fn could_equal(&self, total: usize) -> bool {
        if self.total == total {
            return true;
        }
        if let Some(branches) = &self.branches {
            return branches.add.could_equal(total)
                || branches.multiply.could_equal(total)
                || branches.concatenate.could_equal(total);
        }
        return false;
    }
}

struct Equation {
    total: usize,
    sequence: Vec<usize>,
}

impl Equation {
    fn can_brute_force(&self) -> bool {
        let mut solution_tree = SolutionTree::new(self.sequence[0]);
        for next_value in &self.sequence[1..] {
            solution_tree.next(*next_value);
        }
        solution_tree.could_equal(self.total)
    }

    fn can_brute_force_part2(&self) -> bool {
        let mut solution_tree = SolutionTreePart2::new(self.sequence[0]);
        for next_value in &self.sequence[1..] {
            solution_tree.next(*next_value);
        }
        solution_tree.could_equal(self.total)
    }
}

fn parse_equation(input: &str) -> IResult<&str, Equation, ErrorTree<&str>> {
    let (remainder, (total, sequence)) = separated_pair(
        digit1.map_res(str::parse),
        tag(": "),
        separated_list1(space1, digit1.map_res(str::parse)),
    )(input)?;
    Ok((remainder, Equation { total, sequence }))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_brute_force() {
        let input = "190: 10 19";
        let equation = parse_equation(input).unwrap().1;
        assert_eq!(equation.can_brute_force(), true);
        assert_eq!(equation.total, 190);
    }

    #[test]
    fn test_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(AdventPuzzle::part1(input), "3749");
    }

    #[test]
    fn test_part2() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(AdventPuzzle::part2(input), "11387");
    }
}
