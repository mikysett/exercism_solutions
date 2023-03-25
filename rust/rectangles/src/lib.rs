use Direction::*;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next(&self) -> Option<Self> {
        match self {
            North => None,
            East => Some(South),
            South => Some(West),
            West => Some(North),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Point(usize, usize);

impl Point {
    fn next(&self, dir: &Direction) -> Option<Self> {
        match dir {
            North => Some(Self(self.0, self.1 + 1)),
            East => Some(Self(self.0 + 1, self.1)),
            South => {
                if self.1 == 0 {
                    None
                } else {
                    Some(Self(self.0, self.1 - 1))
                }
            }
            West => {
                if self.0 == 0 {
                    None
                } else {
                    Some(Self(self.0 - 1, self.1))
                }
            }
        }
    }
}

pub fn count(lines: &[&str]) -> u32 {
    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '+')
                .map(|(x, _)| nb_rect_in_line(&Point(x, y), &Point(x + 1, y), East, lines))
                .sum::<u32>()
        })
        .sum()
}

fn nb_rect_in_line(start: &Point, curr: &Point, dir: Direction, map: &[&str]) -> u32 {
    if curr.0 >= map[0].len()
        || curr.0 < start.0
        || curr.1 > start.1
        || (dir == North && curr.0 != start.0)
    {
        return 0;
    }

    let curr_char = map[curr.1].chars().nth(curr.0).unwrap();
    if curr_char == ' '
        || (curr_char == '-' && (dir == Direction::South || dir == Direction::North))
        || (curr_char == '|' && (dir == Direction::East || dir == Direction::West))
    {
        return 0;
    } else if start == curr {
        return 1;
    }

    let mut count = 0;
    if curr_char == '+' {
        if let Some((next_dir, Some(next_point))) =
            dir.next().map(|next_dir| (next_dir, curr.next(&next_dir)))
        {
            count += nb_rect_in_line(start, &next_point, next_dir, map);
        }
    }
    if let Some(next_point) = curr.next(&dir) {
        count += nb_rect_in_line(start, &next_point, dir, map);
    }
    count
}
