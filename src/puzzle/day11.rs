use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::puzzle::day11::Space::{AvailableSeat, Floor, OccupiedSeat};
use crate::puzzle::input::read_input;
use itertools::__std_iter::successors;

pub fn execute() {
    let area = WaitingArea::from(read_input("day11").as_str());
    println!(
        "11:1 — Number of occupied seats after stabilization: {}",
        compute_stable_area(&area).nb_occupied_seats()
    );
    println!(
        "11:1 — Number of occupied seats after stabilization with second part rules: {}",
        compute_stable_area_part2(&area).nb_occupied_seats()
    );
}

fn compute_stable_area(area: &WaitingArea) -> WaitingArea {
    let mut current_area = area.next_round();
    loop {
        let new_area = current_area.next_round();
        if new_area == current_area {
            break current_area;
        }
        current_area = new_area;
    }
}

fn compute_stable_area_part2(area: &WaitingArea) -> WaitingArea {
    let mut current_area = area.next_round_part2();
    loop {
        let new_area = current_area.next_round_part2();
        if new_area == current_area {
            break current_area;
        }
        current_area = new_area;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Space {
    Floor,
    AvailableSeat,
    OccupiedSeat,
}

type Row = Vec<Space>;

#[derive(PartialEq)]
struct WaitingArea {
    rows: Vec<Row>,
}

impl WaitingArea {
    fn next_round(&self) -> Self {
        Self {
            rows: self
                .rows
                .iter()
                .enumerate()
                .map(|(r, row)| {
                    row.iter()
                        .enumerate()
                        .map(
                            |(c, space)| match (space, self.nb_occupied_adjacent_seats(r, c)) {
                                (AvailableSeat, 0) => OccupiedSeat,
                                (OccupiedSeat, n) if n >= 4 => AvailableSeat,
                                _ => *space,
                            },
                        )
                        .collect()
                })
                .collect(),
        }
    }

    fn next_round_part2(&self) -> Self {
        Self {
            rows: self
                .rows
                .iter()
                .enumerate()
                .map(|(r, row)| {
                    row.iter()
                        .enumerate()
                        .map(
                            |(c, space)| match (space, self.nb_visible_occupied_seats(r, c)) {
                                (AvailableSeat, 0) => OccupiedSeat,
                                (OccupiedSeat, n) if n >= 5 => AvailableSeat,
                                _ => *space,
                            },
                        )
                        .collect()
                })
                .collect(),
        }
    }

    fn get_space(&self, row: usize, column: usize) -> Option<&Space> {
        self.rows.get(row).and_then(|row| row.get(column))
    }

    fn is_occupied(&self, row: i32, column: i32) -> bool {
        if row < 0 || column < 0 {
            false
        } else {
            matches!(
                self.get_space(row as usize, column as usize),
                Some(&Space::OccupiedSeat)
            )
        }
    }

    fn nb_occupied_adjacent_seats(&self, row: usize, column: usize) -> usize {
        let row = row as i32;
        let column = column as i32;
        (row - 1..=row + 1)
            .cartesian_product(column - 1..=column + 1)
            .filter(|(r, c)| (*r, *c) != (row, column) && self.is_occupied(*r, *c))
            .count()
    }

    fn has_visible_occupied_seat_in_slope(
        &self,
        row: usize,
        column: usize,
        slope: (i32, i32),
    ) -> bool {
        successors(Some((row as i32, column as i32)), |(r, c)| {
            let next = (r + slope.0, c + slope.1);
            if self.is_in_area(next) {
                Some(next)
            } else {
                None
            }
        })
        .skip(1)
        .flat_map(|(r, c)| self.get_space(r as usize, c as usize))
        .find(|space| **space != Floor)
        .map(|space| *space == OccupiedSeat)
        .unwrap_or(false)
    }

    fn is_in_area(&self, (row, column): (i32, i32)) -> bool {
        row >= 0
            && column >= 0
            && (row as usize) < self.rows.len()
            && (column as usize) < self.rows.get(0).map(|c| c.len()).unwrap_or(0)
    }

    fn nb_visible_occupied_seats(&self, row: usize, column: usize) -> usize {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|slope| *slope != (0, 0))
            .filter(|slope| self.has_visible_occupied_seat_in_slope(row, column, *slope))
            .count()
    }

    fn nb_occupied_seats(&self) -> usize {
        self.rows
            .iter()
            .flat_map(|row| row.iter())
            .filter(|s| **s == OccupiedSeat)
            .count()
    }
}

impl From<&str> for WaitingArea {
    fn from(lines: &str) -> Self {
        let rows = lines
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'L' => Space::AvailableSeat,
                        '#' => Space::OccupiedSeat,
                        _ => Space::Floor,
                    })
                    .collect()
            })
            .collect();
        Self { rows }
    }
}

impl Display for WaitingArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for space in row {
                match space {
                    Space::Floor => write!(f, ".")?,
                    Space::AvailableSeat => write!(f, "L")?,
                    Space::OccupiedSeat => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod waiting_area_is_occupied_should {
    use super::*;

    #[test]
    fn return_true_when_space_is_occupied() {
        let area = WaitingArea::from("#");
        assert!(area.is_occupied(0, 0))
    }

    #[test]
    fn return_false_when_space_is_a_floor() {
        let area = WaitingArea::from(".");
        assert!(!area.is_occupied(0, 0))
    }

    #[test]
    fn return_false_when_space_is_available() {
        let area = WaitingArea::from("L");
        assert!(!area.is_occupied(0, 0))
    }

    #[test]
    fn return_false_when_space_is_out_of_bounds() {
        let area = WaitingArea::from("#");
        assert!(!area.is_occupied(-1, 0))
    }
}

#[cfg(test)]
mod nb_occupied_adjacent_seats_should {
    use super::*;

    #[test]
    fn return_0_when_there_are_no_adjacent_occupied_seats_and_current_seat_is_occupied() {
        let area = WaitingArea::from(
            r"
LLL
L#L
LLL",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(1, 1), 0);
    }

    #[test]
    fn return_1_when_upper_seat_is_occupied() {
        let area = WaitingArea::from(
            r"
L#L
LLL
LLL",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(1, 1), 1);
    }

    #[test]
    fn return_1_when_left_seat_is_occupied() {
        let area = WaitingArea::from(
            r"
LLL
#LL
LLL",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(1, 1), 1);
    }

    #[test]
    fn return_1_when_right_seat_is_occupied() {
        let area = WaitingArea::from(
            r"
LLL
LL#
LLL",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(1, 1), 1);
    }

    #[test]
    fn return_1_when_lower_seat_is_occupied() {
        let area = WaitingArea::from(
            r"
LLL
LLL
L#L",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(1, 1), 1);
    }

    #[test]
    fn return_1_when_upper_left_seat_is_occupied() {
        let area = WaitingArea::from(
            r"
#LL
LLL
LLL",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(1, 1), 1);
    }

    #[test]
    fn return_1_when_upper_right_seat_is_occupied() {
        let area = WaitingArea::from(
            r"
LL#
LLL
LLL",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(1, 1), 1);
    }

    #[test]
    fn return_1_when_lower_left_seat_is_occupied() {
        let area = WaitingArea::from(
            r"
LLL
LLL
#LL",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(1, 1), 1);
    }

    #[test]
    fn return_1_when_lower_right_seat_is_occupied() {
        let area = WaitingArea::from(
            r"
LLL
LLL
LL#",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(1, 1), 1);
    }

    #[test]
    fn return_7_when_there_are_seven_adjacent_spaces() {
        let area = WaitingArea::from(
            r"
###
L##
###",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(1, 1), 7);
    }

    #[test]
    fn return_3_when_current_space_is_in_upper_left_corner_and_all_seats_are_occupied() {
        let area = WaitingArea::from(
            r"
##
##",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(0, 0), 3);
    }

    #[test]
    fn return_3_when_current_space_is_in_upper_right_corner_and_all_seats_are_occupied() {
        let area = WaitingArea::from(
            r"
##
##",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(0, 1), 3);
    }

    #[test]
    fn return_3_when_current_space_is_in_lower_right_corner_and_all_seats_are_occupied() {
        let area = WaitingArea::from(
            r"
##
##",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(1, 1), 3);
    }

    #[test]
    fn return_3_when_current_space_is_in_lower_left_corner_and_all_seats_are_occupied() {
        let area = WaitingArea::from(
            r"
##
##",
        );

        assert_eq!(area.nb_occupied_adjacent_seats(1, 0), 3);
    }
}

#[cfg(test)]
mod next_round_test {
    use super::*;

    #[test]
    fn first_round_of_example() {
        let area = WaitingArea::from(
            r"
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
",
        );

        let next = area.next_round();

        assert_eq!(
            r"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
",
            next.to_string(),
        );
    }

    #[test]
    fn second_round_of_example() {
        let area = WaitingArea::from(
            r"
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
",
        );

        let next = area.next_round();

        assert_eq!(
            r"#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
",
            next.to_string(),
        );
    }

    #[test]
    fn third_round_of_example() {
        let area = WaitingArea::from(
            r"
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
",
        );

        let next = area.next_round();

        assert_eq!(
            r"#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##
",
            next.to_string(),
        );
    }
}

#[cfg(test)]
mod compute_stable_area_test {
    use super::*;

    #[test]
    fn example() {
        let area = WaitingArea::from(
            r"
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
",
        );

        let result = compute_stable_area(&area);

        assert_eq!(
            result.to_string(),
            r"#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
"
        )
    }
}

#[cfg(test)]
mod nb_visible_occupied_seats_should {
    use super::*;

    #[test]
    fn return_8_in_first_example() {
        let area = WaitingArea::from(
            r"
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....
",
        );

        assert_eq!(area.nb_visible_occupied_seats(4, 3), 8);
    }

    #[test]
    fn return_0_in_second_example() {
        let area = WaitingArea::from(
            r"
.............
.L.L.#.#.#.#.
.............
",
        );

        assert_eq!(area.nb_visible_occupied_seats(1, 1), 0);
    }

    #[test]
    fn return_0_in_third_example() {
        let area = WaitingArea::from(
            r"
.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.
",
        );

        assert_eq!(area.nb_visible_occupied_seats(3, 3), 0);
    }
}
