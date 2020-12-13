use super::Action;
use crate::puzzle::day12::Direction;

pub fn execute(actions: &[Action]) {
    let ship = Ship::new();
    let Ship { position, .. } = ship.execute_all(&actions);
    println!(
        "12:2 — Manhattan distance from origin after executing all actions: {}",
        position.manhattan_distance_from_origin(),
    );
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Ship {
    position: Position,
    waypoint: WayPoint,
}

impl Ship {
    fn new() -> Self {
        Self {
            position: Position::new(0, 0),
            waypoint: WayPoint::new(10, 1),
        }
    }

    fn execute(&self, action: Action) -> Self {
        match action {
            Action::Forward(distance) => Self {
                position: self.position.move_forward(&self.waypoint, distance),
                ..*self
            },
            Action::East(distance) => Self {
                waypoint: self.waypoint.translate(Direction::East, distance),
                ..*self
            },
            Action::South(distance) => Self {
                waypoint: self.waypoint.translate(Direction::South, distance),
                ..*self
            },
            Action::West(distance) => Self {
                waypoint: self.waypoint.translate(Direction::West, distance),
                ..*self
            },
            Action::North(distance) => Self {
                waypoint: self.waypoint.translate(Direction::North, distance),
                ..*self
            },
            Action::Right(angle) => Self {
                waypoint: self.waypoint.turn(angle),
                ..*self
            },
            Action::Left(angle) => Self {
                waypoint: self.waypoint.turn(-angle),
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

#[derive(Copy, Clone, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_forward(&self, waypoint: &WayPoint, times: i32) -> Self {
        Self {
            x: self.x + waypoint.x * times,
            y: self.y + waypoint.y * times,
        }
    }

    fn manhattan_distance_from_origin(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct WayPoint {
    x: i32,
    y: i32,
}

impl WayPoint {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn turn(&self, angle: i32) -> Self {
        match (angle / 90) % 4 {
            0 => *self,
            1 | -3 => Self {
                x: self.y,
                y: -self.x,
            },
            2 | -2 => Self {
                x: -self.x,
                y: -self.y,
            },
            _ => Self {
                x: -self.y,
                y: self.x,
            },
        }
    }

    fn translate(&self, direction: Direction, distance: i32) -> Self {
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
}
