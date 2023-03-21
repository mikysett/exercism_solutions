// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use Direction::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (i32, i32),
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            pos: (x, y),
            dir: d,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            dir: match self.dir {
                North => East,
                East => South,
                South => West,
                West => North,
            },
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            dir: match self.dir {
                North => West,
                East => North,
                South => East,
                West => South,
            },
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.dir {
            North => Self { pos: (self.pos.0, self.pos.1 + 1), ..self },
            East => Self { pos: (self.pos.0 + 1, self.pos.1), ..self },
            South => Self { pos: (self.pos.0, self.pos.1 + -1), ..self },
            West => Self { pos: (self.pos.0 - 1, self.pos.1), ..self },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'A' => robot.advance(),
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                _ => panic!("Bad instruction"),
            })
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
