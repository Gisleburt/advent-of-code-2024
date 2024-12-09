use crate::solution::Solution;
use nom::branch::alt;
use nom::character::complete::newline;
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::{IResult, Parser};
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct AdventPuzzle;

impl Solution for AdventPuzzle {
    fn part1(input: &str) -> String {
        let (_, mut puzzle) = parse_puzzle(input).unwrap();
        while puzzle.process() == PuzzleResult::Continue {}
        puzzle.count_visited().to_string()
    }

    fn part2(input: &str) -> String {
        let (_, mut puzzle) = parse_puzzle(input).unwrap();
        let initial_state = puzzle.clone();

        while puzzle.process() == PuzzleResult::Continue {}

        let positions = puzzle.get_all_visited();
        positions
            .iter()
            .skip(1)
            .filter(|pos| {
                let mut new_puzzle = initial_state.clone();
                new_puzzle.add_obstacle(pos);

                loop {
                    let final_result = new_puzzle.process();
                    if final_result != PuzzleResult::Continue {
                        return final_result == PuzzleResult::Looped;
                    }
                }
            })
            .count()
            .to_string()
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

impl GuardDirection {
    fn turn_right(self) -> GuardDirection {
        match self {
            GuardDirection::Up => GuardDirection::Right,
            GuardDirection::Right => GuardDirection::Down,
            GuardDirection::Down => GuardDirection::Left,
            GuardDirection::Left => GuardDirection::Up,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct GuardPosition {
    row: isize,
    column: isize,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Position {
    Empty,
    Visited,
    Obstructed,
}

fn parse_position(input: &str) -> IResult<&str, Position, ErrorTree<&str>> {
    alt((
        tag(".").map(|_| Position::Empty),
        tag("#").map(|_| Position::Obstructed),
        tag("^").map(|_| Position::Visited),
    ))(input)
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum PuzzleInputError {
    InvalidSize,
    MissingStart,
    TooManyStarts,
}

impl Display for PuzzleInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PuzzleInputError::InvalidSize => write!(f, "invalid size"),
            PuzzleInputError::MissingStart => write!(f, "missing start"),
            PuzzleInputError::TooManyStarts => write!(f, "too many starts"),
        }
    }
}

impl Error for PuzzleInputError {}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Bump {
    pos: GuardPosition,
    dir: GuardDirection,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum PuzzleResult {
    Continue,
    Exited,
    Looped,
}

#[derive(Clone, PartialEq, Debug)]
struct Puzzle {
    map: Vec<Vec<Position>>,
    bumps: Vec<Bump>,
    guard_position: GuardPosition,
    guard_direction: GuardDirection,
}

impl Puzzle {
    fn new(map: Vec<Vec<Position>>) -> Result<Self, PuzzleInputError> {
        let mut guard_position: Option<GuardPosition> = None;
        let width = map[0].len();

        for (r, row) in map.iter().enumerate() {
            if row.len() != width {
                return Err(PuzzleInputError::InvalidSize);
            }
            for (c, pos) in row.iter().enumerate() {
                if *pos == Position::Visited {
                    if guard_position.is_some() {
                        return Err(PuzzleInputError::TooManyStarts);
                    }
                    guard_position = Some(GuardPosition {
                        row: r as isize,
                        column: c as isize,
                    });
                }
            }
        }

        Ok(Puzzle {
            map,
            bumps: Vec::new(),
            guard_position: guard_position.ok_or(PuzzleInputError::MissingStart)?,
            guard_direction: GuardDirection::Up,
        })
    }

    fn process(&mut self) -> PuzzleResult {
        let (row, col) = match self.guard_direction {
            GuardDirection::Up => (-1, 0),
            GuardDirection::Down => (1, 0),
            GuardDirection::Left => (0, -1),
            GuardDirection::Right => (0, 1),
        };

        let new_position = GuardPosition {
            row: self.guard_position.row + row,
            column: self.guard_position.column + col,
        };

        if !self.is_in_bounds(new_position) {
            return PuzzleResult::Exited;
        }

        if self.map[new_position.row as usize][new_position.column as usize] == Position::Obstructed
        {
            let bump = Bump {
                pos: new_position,
                dir: self.guard_direction,
            };

            if self.bumps.contains(&bump) {
                return PuzzleResult::Looped;
            }

            self.bumps.push(bump);
            self.guard_direction = self.guard_direction.turn_right();
            return PuzzleResult::Continue;
        }

        self.map[new_position.row as usize][new_position.column as usize] = Position::Visited;
        self.guard_position = new_position;
        PuzzleResult::Continue
    }

    fn is_in_bounds(&self, GuardPosition { row, column }: GuardPosition) -> bool {
        row >= 0
            && row < self.map.len() as isize
            && column >= 0
            && column < self.map[0].len() as isize
    }

    fn count_visited(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|p| **p == Position::Visited)
            .count()
    }

    fn get_all_visited(&self) -> Vec<GuardPosition> {
        self.map
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter().enumerate().map(move |(c, _pos)| GuardPosition {
                    row: r as isize,
                    column: c as isize,
                })
            })
            .flatten()
            .collect()
    }

    fn add_obstacle(&mut self, GuardPosition { row, column }: &GuardPosition) {
        self.map[*row as usize][*column as usize] = Position::Obstructed;
    }
}

fn parse_puzzle(input: &str) -> IResult<&str, Puzzle, ErrorTree<&str>> {
    map_res(separated_list1(newline, many1(parse_position)), Puzzle::new)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        assert_eq!(AdventPuzzle::part1(input), "41");
    }

    #[test]
    fn test_part2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        assert_eq!(AdventPuzzle::part2(input), "6");
    }
}
