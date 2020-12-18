use std::collections::HashSet;
use std::ops::RangeInclusive;

use crate::puzzle::input::read_input;
use itertools::Itertools;

pub fn execute() {
    let input = read_input("day17");
    let mut cubes = ConwayCubes::from(&input);
    for _ in 0..6 {
        cubes = cubes.next_cycle()
    }
    println!(
        "17:1 — Number of cubes after 6 cycles: {}",
        cubes.count_cubes()
    );
}

struct ConwayCubes {
    cubes: HashSet<(i32, i32, i32)>,
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl ConwayCubes {
    fn new(cubes: HashSet<(i32, i32, i32)>) -> Self {
        let x = cubes.iter().map(|(x, _, _)| x).into_range();
        let y = cubes.iter().map(|(_, y, _)| y).into_range();
        let z = cubes.iter().map(|(_, _, z)| z).into_range();
        Self { cubes, x, y, z }
    }

    fn is_active(&self, position: &(i32, i32, i32)) -> bool {
        self.cubes.iter().any(|p| p == position)
    }

    fn count_adjacent_cubes(&self, position: &(i32, i32, i32)) -> usize {
        (position.0 - 1..=position.0 + 1)
            .cartesian_product(position.1 - 1..=position.1 + 1)
            .cartesian_product(position.2 - 1..=position.2 + 1)
            .map(|((x, y), z)| (x, y, z))
            .filter(|p| p != position)
            .filter(|p| self.cubes.contains(p))
            .count()
    }

    fn next_cycle(&self) -> Self {
        let cubes = self
            .x
            .expand()
            .cartesian_product(self.y.expand())
            .cartesian_product(self.z.expand())
            .map(|((x, y), z)| (x, y, z))
            .filter(|p| {
                let adjacent_cubes = self.count_adjacent_cubes(p);
                adjacent_cubes == 3 || self.is_active(p) && adjacent_cubes == 2
            })
            .collect();
        Self::new(cubes)
    }

    fn count_cubes(&self) -> usize {
        self.cubes.len()
    }
}

trait IntoRange<T> {
    fn into_range(self) -> RangeInclusive<T>;
}

impl<'a, I> IntoRange<i32> for I
where
    I: Iterator<Item = &'a i32>,
{
    fn into_range(self) -> RangeInclusive<i32> {
        self.copied()
            .minmax()
            .to_owned()
            .into_option()
            .map(|(s, e)| s..=e)
            .unwrap_or(0..=0)
    }
}

trait Expand {
    fn expand(&self) -> Self;
}

impl Expand for RangeInclusive<i32> {
    fn expand(&self) -> Self {
        self.start() - 1..=self.end() + 1
    }
}

impl From<&str> for ConwayCubes {
    fn from(first_slice: &str) -> Self {
        let cubes = first_slice
            .lines()
            .filter(|line| !line.is_empty())
            .enumerate()
            // .map(|(i, line)| (i as i32, to_line(line)))
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '#')
                    .map(move |(x, _)| (x as i32, y as i32, 0))
            })
            .collect();
        Self::new(cubes)
    }
}

impl From<&String> for ConwayCubes {
    fn from(first_slice: &String) -> Self {
        first_slice.as_str().into()
    }
}

#[cfg(test)]
static EXAMPLE: &str = "
.#.
..#
###
";

#[cfg(test)]
mod conway_cubes_from_should {
    use super::*;

    #[test]
    fn parse_example() {
        let slice = ConwayCubes::from(EXAMPLE);
        assert_eq!(
            slice.cubes,
            vec![(1, 0, 0), (2, 1, 0), (0, 2, 0), (1, 2, 0), (2, 2, 0)]
                .into_iter()
                .collect::<HashSet<(i32, i32, i32)>>()
        );
    }
}

#[cfg(test)]
mod conway_cubes_count_adjacent_cubes_should {
    use super::*;

    #[test]
    fn return_1_for_0_0_in_the_example() {
        let slice = ConwayCubes::from(EXAMPLE);

        assert_eq!(slice.count_adjacent_cubes(&(0, 0, 0)), 1);
    }

    #[test]
    fn return_1_for_1_0_in_the_example() {
        let slice = ConwayCubes::from(EXAMPLE);

        assert_eq!(slice.count_adjacent_cubes(&(1, 0, 0)), 1);
    }

    #[test]
    fn return_5_for_1_1_in_the_example() {
        let slice = ConwayCubes::from(EXAMPLE);

        assert_eq!(slice.count_adjacent_cubes(&(1, 1, 0)), 5);
    }
}

#[cfg(test)]
mod text {
    use super::*;

    #[test]
    fn example() {
        let mut cubes = ConwayCubes::from(EXAMPLE);
        for _ in 0..6 {
            cubes = cubes.next_cycle();
        }
        assert_eq!(cubes.count_cubes(), 112)
    }
}
