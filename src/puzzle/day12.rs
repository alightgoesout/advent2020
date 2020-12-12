use crate::puzzle::input::read_lines;

pub fn execute() {
    let actions = get_actions();
    let ship = Ship::new();
    let Ship { position, .. } = ship.execute_all(&actions);
    println!(
        "12:1 — Manhattan distance from origin after executing all actions: {}",
        position.manhattan_distance_from_origin(),
    )
}

fn get_actions() -> Vec<Action> {
    read_lines("day12")
        .unwrap()
        .iter()
        .map(|line| line.as_str().into())
        .collect()
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn turn(&self, angle: i32) -> Self {
        match (self.value() + angle / 90) % 4 {
            0 => Direction::North,
            1 | -3 => Direction::East,
            2 | -2 => Direction::South,
            _ => Direction::West,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_forward(&self, direction: Direction, distance: i32) -> Self {
        match direction {
            Direction::East => Self {
                x: self.x + distance,
                ..*self
            },
            Direction::South => Self {
                y: self.y - distance,
                ..*self
            },
            Direction::West => Self {
                x: self.x - distance,
                ..*self
            },
            Direction::North => Self {
                y: self.y + distance,
                ..*self
            },
        }
    }

    fn manhattan_distance_from_origin(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Ship {
    direction: Direction,
    position: Position,
}

impl Ship {
    fn new() -> Self {
        Self {
            direction: Direction::East,
            position: Position::new(0, 0),
        }
    }

    fn execute(&self, action: Action) -> Self {
        match action {
            Action::Forward(distance) => Self {
                position: self.position.move_forward(self.direction, distance),
                ..*self
            },
            Action::East(distance) => Self {
                position: self.position.move_forward(Direction::East, distance),
                ..*self
            },
            Action::South(distance) => Self {
                position: self.position.move_forward(Direction::South, distance),
                ..*self
            },
            Action::West(distance) => Self {
                position: self.position.move_forward(Direction::West, distance),
                ..*self
            },
            Action::North(distance) => Self {
                position: self.position.move_forward(Direction::North, distance),
                ..*self
            },
            Action::Right(angle) => Self {
                direction: self.direction.turn(angle),
                ..*self
            },
            Action::Left(angle) => Self {
                direction: self.direction.turn(-angle),
                ..*self
            },
        }
    }

    fn execute_all(&self, actions: &[Action]) -> Ship {
        actions
            .iter()
            .fold(*self, |ship, action| ship.execute(*action))
    }
}

#[derive(Copy, Clone)]
enum Action {
    North(i32),
    South(i32),
    West(i32),
    East(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl From<&str> for Action {
    fn from(action: &str) -> Self {
        let value = action[1..].parse().unwrap();
        match &action[0..1] {
            "N" => Action::North(value),
            "S" => Action::South(value),
            "W" => Action::West(value),
            "E" => Action::East(value),
            "L" => Action::Left(value),
            "R" => Action::Right(value),
            "F" => Action::Forward(value),
            c => panic!("Unknown action {}", c),
        }
    }
}

#[cfg(test)]
mod ship_execute_should {
    use super::*;

    #[test]
    fn move_the_ship_10_towards_east_when_it_is_facing_east_and_action_is_forward_10() {
        let ship = Ship {
            direction: Direction::East,
            position: Position::new(0, 0),
        };

        let result = ship.execute(Action::Forward(10));

        assert_eq!(
            result,
            Ship {
                direction: Direction::East,
                position: Position::new(10, 0)
            }
        );
    }

    #[test]
    fn move_the_ship_10_towards_south_when_it_is_facing_south_and_action_is_forward_10() {
        let ship = Ship {
            direction: Direction::South,
            position: Position::new(0, 0),
        };

        let result = ship.execute(Action::Forward(10));

        assert_eq!(
            result,
            Ship {
                direction: Direction::South,
                position: Position::new(0, -10)
            }
        );
    }

    #[test]
    fn move_the_ship_10_towards_west_when_it_is_facing_west_and_action_is_forward_10() {
        let ship = Ship {
            direction: Direction::West,
            position: Position::new(0, 0),
        };

        let result = ship.execute(Action::Forward(10));

        assert_eq!(
            result,
            Ship {
                direction: Direction::West,
                position: Position::new(-10, 0)
            }
        );
    }

    #[test]
    fn move_the_ship_10_towards_north_when_it_is_facing_north_and_action_is_forward_10() {
        let ship = Ship {
            direction: Direction::North,
            position: Position::new(0, 0),
        };

        let result = ship.execute(Action::Forward(10));

        assert_eq!(
            result,
            Ship {
                direction: Direction::North,
                position: Position::new(0, 10)
            }
        );
    }

    #[test]
    fn move_the_ship_10_towards_east_when_action_is_east_10() {
        let ship = Ship {
            direction: Direction::West,
            position: Position::new(0, 0),
        };

        let result = ship.execute(Action::East(10));

        assert_eq!(
            result,
            Ship {
                direction: Direction::West,
                position: Position::new(10, 0)
            }
        );
    }

    #[test]
    fn move_the_ship_10_towards_south_when_action_is_south_10() {
        let ship = Ship::new();

        let result = ship.execute(Action::South(10));

        assert_eq!(
            result,
            Ship {
                direction: Direction::East,
                position: Position::new(0, -10)
            }
        );
    }

    #[test]
    fn move_the_ship_10_towards_west_when_action_is_west_10() {
        let ship = Ship::new();

        let result = ship.execute(Action::West(10));

        assert_eq!(
            result,
            Ship {
                direction: Direction::East,
                position: Position::new(-10, 0)
            }
        );
    }

    #[test]
    fn move_the_ship_10_towards_north_when_action_is_north_10() {
        let ship = Ship::new();

        let result = ship.execute(Action::North(10));

        assert_eq!(
            result,
            Ship {
                direction: Direction::East,
                position: Position::new(0, 10)
            }
        );
    }

    #[test]
    fn turn_the_ship_towards_south_when_it_is_facing_east_and_action_is_right_90() {
        let ship = Ship {
            direction: Direction::East,
            position: Position::new(0, 0),
        };

        let result = ship.execute(Action::Right(90));

        assert_eq!(
            result,
            Ship {
                direction: Direction::South,
                position: Position::new(0, 0)
            }
        );
    }

    #[test]
    fn turn_the_ship_towards_north_when_it_is_facing_east_and_action_is_right_270() {
        let ship = Ship {
            direction: Direction::East,
            position: Position::new(0, 0),
        };

        let result = ship.execute(Action::Right(270));

        assert_eq!(
            result,
            Ship {
                direction: Direction::North,
                position: Position::new(0, 0)
            }
        );
    }

    #[test]
    fn turn_the_ship_towards_north_when_it_is_facing_east_and_action_is_left_90() {
        let ship = Ship {
            direction: Direction::East,
            position: Position::new(0, 0),
        };

        let result = ship.execute(Action::Left(90));

        assert_eq!(
            result,
            Ship {
                direction: Direction::North,
                position: Position::new(0, 0)
            }
        );
    }

    #[test]
    fn turn_the_ship_towards_south_when_it_is_facing_east_and_action_is_left_270() {
        let ship = Ship {
            direction: Direction::East,
            position: Position::new(0, 0),
        };

        let result = ship.execute(Action::Left(270));

        assert_eq!(
            result,
            Ship {
                direction: Direction::South,
                position: Position::new(0, 0)
            }
        );
    }
}
