use crate::solution::Solution;
use std::str::FromStr;

pub struct AdventPuzzle;

impl Solution for AdventPuzzle {
    fn part1(input: &str) -> String {
        let grid = Grid::from_str(input).unwrap();
        grid.count_xmas().to_string()
    }

    fn part2(input: &str) -> String {
        let grid = Grid::from_str(input).unwrap();
        grid.count_x_mas().to_string()
    }
}

// enum Direction {
//     N,
//     NE,
//     E,
//     SE,
//     S,
//     SW,
//     W,
//     NW,
// }

pub struct Grid(Vec<Vec<char>>);

impl Grid {
    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn is_xmas_n(&self, row: usize, col: usize) -> bool {
        row >= 3
            && row < self.height()
            && col < self.width()
            && self.0.get(row).and_then(|row| row.get(col)) == Some(&'X')
            && self.0.get(row - 1).and_then(|row| row.get(col)) == Some(&'M')
            && self.0.get(row - 2).and_then(|row| row.get(col)) == Some(&'A')
            && self.0.get(row - 3).and_then(|row| row.get(col)) == Some(&'S')
    }
    fn is_xmas_ne(&self, row: usize, col: usize) -> bool {
        row >= 3
            && row < self.height()
            && col < self.width() - 3
            && self.0.get(row).and_then(|row| row.get(col)) == Some(&'X')
            && self.0.get(row - 1).and_then(|row| row.get(col + 1)) == Some(&'M')
            && self.0.get(row - 2).and_then(|row| row.get(col + 2)) == Some(&'A')
            && self.0.get(row - 3).and_then(|row| row.get(col + 3)) == Some(&'S')
    }
    fn is_xmas_e(&self, row: usize, col: usize) -> bool {
        row < self.height()
            && col < self.width() - 3
            && self.0.get(row).and_then(|row| row.get(col)) == Some(&'X')
            && self.0.get(row).and_then(|row| row.get(col + 1)) == Some(&'M')
            && self.0.get(row).and_then(|row| row.get(col + 2)) == Some(&'A')
            && self.0.get(row).and_then(|row| row.get(col + 3)) == Some(&'S')
    }
    fn is_xmas_se(&self, row: usize, col: usize) -> bool {
        row < self.height() - 3
            && col < self.width() - 3
            && self.0.get(row).and_then(|row| row.get(col)) == Some(&'X')
            && self.0.get(row + 1).and_then(|row| row.get(col + 1)) == Some(&'M')
            && self.0.get(row + 2).and_then(|row| row.get(col + 2)) == Some(&'A')
            && self.0.get(row + 3).and_then(|row| row.get(col + 3)) == Some(&'S')
    }
    fn is_xmas_s(&self, row: usize, col: usize) -> bool {
        row < self.height() - 3
            && col < self.width()
            && self.0.get(row).and_then(|row| row.get(col)) == Some(&'X')
            && self.0.get(row + 1).and_then(|row| row.get(col)) == Some(&'M')
            && self.0.get(row + 2).and_then(|row| row.get(col)) == Some(&'A')
            && self.0.get(row + 3).and_then(|row| row.get(col)) == Some(&'S')
    }
    fn is_xmas_sw(&self, row: usize, col: usize) -> bool {
        col >= 3
            && row < self.height() - 3
            && col < self.width()
            && self.0.get(row).and_then(|row| row.get(col)) == Some(&'X')
            && self.0.get(row + 1).and_then(|row| row.get(col - 1)) == Some(&'M')
            && self.0.get(row + 2).and_then(|row| row.get(col - 2)) == Some(&'A')
            && self.0.get(row + 3).and_then(|row| row.get(col - 3)) == Some(&'S')
    }
    fn is_xmas_w(&self, row: usize, col: usize) -> bool {
        col >= 3
            && row < self.height()
            && col < self.width()
            && self.0.get(row).and_then(|row| row.get(col)) == Some(&'X')
            && self.0.get(row).and_then(|row| row.get(col - 1)) == Some(&'M')
            && self.0.get(row).and_then(|row| row.get(col - 2)) == Some(&'A')
            && self.0.get(row).and_then(|row| row.get(col - 3)) == Some(&'S')
    }
    fn is_xmas_nw(&self, row: usize, col: usize) -> bool {
        row >= 3
            && col >= 3
            && row < self.height()
            && col < self.width()
            && self.0.get(row).and_then(|row| row.get(col)) == Some(&'X')
            && self.0.get(row - 1).and_then(|row| row.get(col - 1)) == Some(&'M')
            && self.0.get(row - 2).and_then(|row| row.get(col - 2)) == Some(&'A')
            && self.0.get(row - 3).and_then(|row| row.get(col - 3)) == Some(&'S')
    }

    fn count_xmas(&self) -> usize {
        let mut total = 0;
        for row in 0..self.0.len() {
            for col in 0..self.0[0].len() {
                total += [
                    self.is_xmas_n(row, col),
                    self.is_xmas_ne(row, col),
                    self.is_xmas_e(row, col),
                    self.is_xmas_se(row, col),
                    self.is_xmas_s(row, col),
                    self.is_xmas_sw(row, col),
                    self.is_xmas_w(row, col),
                    self.is_xmas_nw(row, col),
                ]
                .into_iter()
                .filter(|result| *result)
                .count();
            }
        }
        total
    }

    fn is_x_mas(&self, row: usize, col: usize) -> bool {
        row >= 1
            && col >= 1
            && row < self.height() - 1
            && col < self.width() - 1
            && self.0.get(row).and_then(|row| row.get(col)) == Some(&'A')
            && (self.0.get(row + 1).and_then(|row| row.get(col + 1)) == Some(&'M')
                && self.0.get(row - 1).and_then(|row| row.get(col - 1)) == Some(&'S')
                || self.0.get(row - 1).and_then(|row| row.get(col - 1)) == Some(&'M')
                    && self.0.get(row + 1).and_then(|row| row.get(col + 1)) == Some(&'S'))
            && (self.0.get(row + 1).and_then(|row| row.get(col - 1)) == Some(&'M')
                && self.0.get(row - 1).and_then(|row| row.get(col + 1)) == Some(&'S')
                || self.0.get(row - 1).and_then(|row| row.get(col + 1)) == Some(&'M')
                    && self.0.get(row + 1).and_then(|row| row.get(col - 1)) == Some(&'S'))
    }

    fn count_x_mas(&self) -> usize {
        let mut total = 0;
        for row in 1..self.0.len() - 1 {
            for col in 1..self.0[0].len() - 1 {
                total += self.is_x_mas(row, col) as usize;
            }
        }
        total
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Self(
            s.lines()
                .map(|line| line.chars().collect())
                .filter(|l: &Vec<_>| !l.is_empty())
                .collect(),
        );

        let width = grid.0[0].len();
        if grid.0.iter().any(|c| c.len() != width) {
            panic!("invalid grid")
        }

        Ok(grid)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(AdventPuzzle::part1(input), "18");
    }

    #[test]
    fn test_part2() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!(AdventPuzzle::part2(input), "9");
    }
}
