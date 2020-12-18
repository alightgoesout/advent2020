use std::collections::HashSet;
use std::ops::RangeInclusive;

use crate::puzzle::input::read_input;
use itertools::Itertools;

pub fn execute() {
    let input = read_input("day17");
    let mut cubes = ConwayCubes::from(&input, 3);
    for _ in 0..6 {
        cubes = cubes.next_cycle()
    }
    println!(
        "17:1 — Number of cubes after 6 cycles: {}",
        cubes.count_cubes()
    );
    let mut cubes = ConwayCubes::from(&input, 4);
    for _ in 0..6 {
        cubes = cubes.next_cycle()
    }
    println!(
        "17:2 — Number of hypercubes after 6 cycles: {}",
        cubes.count_cubes()
    );
}

struct ConwayCubes {
    cubes: HashSet<Vec<i32>>,
    dimensions: Vec<RangeInclusive<i32>>,
}

impl ConwayCubes {
    fn new(cubes: HashSet<Vec<i32>>, nb_dimensions: usize) -> Self {
        if nb_dimensions < 2 {
            panic!("Minimum two dimensions");
        }
        let dimensions = (0..nb_dimensions)
            .map(|d| cubes.iter().map(|c| c[d]).into_range())
            .collect();
        Self { cubes, dimensions }
    }

    fn from(first_slice: &str, nb_dimensions: usize) -> Self {
        let cubes = first_slice
            .lines()
            .filter(|line| !line.is_empty())
            .enumerate()
            // .map(|(i, line)| (i as i32, to_line(line)))
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '#')
                    .map(move |(x, _)| {
                        let mut p = Vec::with_capacity(nb_dimensions);
                        p.push(x as i32);
                        p.push(y as i32);
                        for _ in 2..nb_dimensions {
                            p.push(0)
                        }
                        p
                    })
            })
            .collect();
        Self::new(cubes, nb_dimensions)
    }

    fn is_active(&self, position: &[i32]) -> bool {
        self.cubes.iter().any(|p| p == &position)
    }

    fn count_adjacent_cubes(&self, position: &[i32]) -> usize {
        position
            .iter()
            .map(|i| i - 1..=i + 1)
            .multi_cartesian_product()
            .filter(|p| p != &position)
            .filter(|p| self.cubes.contains(p))
            .count()
    }

    fn next_cycle(&self) -> Self {
        let cubes = self
            .dimensions
            .iter()
            .map(|d| d.expand())
            .multi_cartesian_product()
            .filter(|p| {
                let adjacent_cubes = self.count_adjacent_cubes(p);
                adjacent_cubes == 3 || self.is_active(p) && adjacent_cubes == 2
            })
            .collect();
        Self::new(cubes, self.dimensions.len())
    }

    fn count_cubes(&self) -> usize {
        self.cubes.len()
    }
}

trait IntoRange<T> {
    fn into_range(self) -> RangeInclusive<T>;
}

impl<I> IntoRange<i32> for I
where
    I: Iterator<Item = i32>,
{
    fn into_range(self) -> RangeInclusive<i32> {
        self.minmax()
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
        let slice = ConwayCubes::from(EXAMPLE, 3);
        assert_eq!(
            slice.cubes,
            vec![
                vec![1, 0, 0],
                vec![2, 1, 0],
                vec![0, 2, 0],
                vec![1, 2, 0],
                vec![2, 2, 0]
            ]
            .into_iter()
            .collect::<HashSet<Vec<i32>>>()
        );
        assert_eq!(slice.count_cubes(), 5);
    }
}

#[cfg(test)]
mod conway_cubes_count_adjacent_cubes_should {
    use super::*;

    #[test]
    fn return_1_for_0_0_in_the_example() {
        let slice = ConwayCubes::from(EXAMPLE, 3);

        assert_eq!(slice.count_adjacent_cubes(&[0, 0, 0]), 1);
    }

    #[test]
    fn return_1_for_1_0_in_the_example() {
        let slice = ConwayCubes::from(EXAMPLE, 3);

        assert_eq!(slice.count_adjacent_cubes(&[1, 0, 0]), 1);
    }

    #[test]
    fn return_5_for_1_1_in_the_example() {
        let slice = ConwayCubes::from(EXAMPLE, 3);

        assert_eq!(slice.count_adjacent_cubes(&[1, 1, 0]), 5);
    }
}

#[cfg(test)]
mod text {
    use super::*;

    #[test]
    fn example_3_dimensions() {
        let mut cubes = ConwayCubes::from(EXAMPLE, 3);
        for _ in 0..6 {
            cubes = cubes.next_cycle();
        }
        assert_eq!(cubes.count_cubes(), 112)
    }

    #[test]
    fn example_4_dimensions() {
        let mut cubes = ConwayCubes::from(EXAMPLE, 4);
        for _ in 0..6 {
            cubes = cubes.next_cycle();
        }
        assert_eq!(cubes.count_cubes(), 848)
    }
}
