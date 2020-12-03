use std::ops::{Add, Rem};

use im_rc::Vector;

use crate::puzzle::input::read_lines;

const TREE_CHAR: char = '#';

pub fn execute() {
    let topology = read_lines("day3").unwrap().into();
    println!(
        "3:1 — Number of trees on slope: {}",
        count_trees_on_slope(&topology, Slope::new(3, 1)),
    );
}

fn count_trees_on_slope(topology: &Topology, slope: Slope) -> usize {
    TobogganDescent::new(topology, slope).filter(|r| *r).count()
}

struct Topology {
    width: usize,
    trees: Vector<TopologyLine>,
}

#[derive(Clone)]
struct TopologyLine(Vector<usize>);

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Slope {
    right: usize,
    down: usize,
}

struct TobogganDescent<'a> {
    topology: &'a Topology,
    slope: Slope,
    position: Position,
}

impl Topology {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.trees.len()
    }

    fn has_tree(&self, position: &Position) -> bool {
        self.trees
            .get(position.y)
            .map(|line| line.has_tree(position.x))
            .unwrap_or(false)
    }
}

impl From<Vector<String>> for Topology {
    fn from(lines: Vector<String>) -> Self {
        Self {
            width: lines.get(0).map(|line| line.len()).unwrap_or(0),
            trees: lines.into_iter().map(|line| line.into()).collect(),
        }
    }
}

impl TopologyLine {
    fn has_tree(&self, position: usize) -> bool {
        self.0.contains(&position)
    }
}

impl From<String> for TopologyLine {
    fn from(line: String) -> Self {
        Self(
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == TREE_CHAR)
                .map(|(i, _)| i)
                .collect(),
        )
    }
}

impl<'a> TobogganDescent<'a> {
    fn new(topology: &'a Topology, slope: Slope) -> Self {
        Self {
            topology,
            slope,
            position: Position::initial(),
        }
    }
}

impl<'a> Iterator for TobogganDescent<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.position.y > self.topology.height() {
            None
        } else {
            Some(self.topology.has_tree(&self.position))
        };
        self.position = (&self.position + &self.slope) % self.topology.width;
        result
    }
}

impl Slope {
    fn new(right: usize, down: usize) -> Self {
        Self { right, down }
    }
}

impl Position {
    fn initial() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Add<&Slope> for &Position {
    type Output = Position;

    fn add(self, slope: &Slope) -> Self::Output {
        Position {
            x: self.x + slope.right,
            y: self.y + slope.down,
        }
    }
}

impl Rem<usize> for Position {
    type Output = Position;

    fn rem(self, width: usize) -> Self::Output {
        Position {
            x: self.x % width,
            y: self.y,
        }
    }
}

#[cfg(test)]
mod count_trees_on_slope_should {
    use im_rc::vector;

    use super::*;

    #[test]
    fn return_0_when_first_line_has_no_trees_and_there_is_one_line() {
        let topology = vector![".....".into()].into();
        let slope = Slope::new(1, 1);

        let result = count_trees_on_slope(&topology, slope);

        assert_eq!(result, 0);
    }

    #[test]
    fn return_1_when_first_line_has_a_tree_in_first_position_and_there_is_one_line() {
        let topology = vector!["#....".into()].into();
        let slope = Slope::new(1, 1);

        let result = count_trees_on_slope(&topology, slope);

        assert_eq!(result, 1);
    }

    #[test]
    fn return_0_when_first_position_and_second_position_have_no_trees_and_there_are_two_lines() {
        let topology = vector!["......".into(), "......".into()].into();
        let slope = Slope::new(1, 1);

        let result = count_trees_on_slope(&topology, slope);

        assert_eq!(result, 0);
    }

    #[test]
    fn return_1_when_first_position_has_tree_and_second_position_has_no_tree_and_there_are_two_lines(
    ) {
        let topology = vector!["#.....".into(), "......".into()].into();
        let slope = Slope::new(1, 1);

        let result = count_trees_on_slope(&topology, slope);

        assert_eq!(result, 1);
    }

    #[test]
    fn return_2_when_first_position_and_second_position_have_trees_and_there_are_two_lines() {
        let topology = vector!["#.....".into(), ".#....".into()].into();
        let slope = Slope::new(1, 1);

        let result = count_trees_on_slope(&topology, slope);

        assert_eq!(result, 2);
    }

    #[test]
    fn return_1_when_there_is_a_tree_on_2_2_slope_is_4_2_and_width_is_8() {
        let topology = vector![
            "......".into(),
            "......".into(),
            "..#...".into(),
            "......".into(),
        ]
        .into();
        let slope = Slope::new(4, 1);

        let result = count_trees_on_slope(&topology, slope);

        assert_eq!(result, 1);
    }

    #[test]
    fn return_7_for_the_example() {
        let topology = vector![
            "..##.......".into(),
            "#...#...#..".into(),
            ".#....#..#.".into(),
            "..#.#...#.#".into(),
            ".#...##..#.".into(),
            "..#.##.....".into(),
            ".#.#.#....#".into(),
            ".#........#".into(),
            "#.##...#...".into(),
            "#...##....#".into(),
            ".#..#...#.#".into(),
        ]
        .into();
        let slope = Slope::new(3, 1);

        let result = count_trees_on_slope(&topology, slope);

        assert_eq!(result, 7);
    }
}
