// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

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
        Self::new(
            self.pos.0,
            self.pos.1,
            match self.dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        )
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self::new(
            self.pos.0,
            self.pos.1,
            match self.dir {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
        )
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let new_pos = match self.dir {
            Direction::North => (self.pos.0, self.pos.1 + 1),
            Direction::East => (self.pos.0 + 1, self.pos.1),
            Direction::South => (self.pos.0, self.pos.1 + -1),
            Direction::West => (self.pos.0 - 1, self.pos.1),
        };
        Self::new(new_pos.0, new_pos.1, self.dir)
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
