struct Point(usize, usize);

pub fn count(lines: &[&str]) -> u32 {
    let board_size = Point(
        lines.first().map(|line| line.len()).unwrap_or(0),
        lines.len(),
    );

    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '+')
                .map(|(x, _)| nb_rect_in_line(&Point(x, y), lines, &board_size))
                .sum::<u32>()
        })
        .sum()
}

fn nb_rect_in_line(start: &Point, board: &[&str], board_size: &Point) -> u32 {
    let mut count = 0;
    for x in start.0 + 1..board_size.0 {
        let curr_char = board[start.1].chars().nth(x).unwrap();
        if curr_char == '+' {
            count += nb_valid_rect(start, x, board, board_size);
        } else if curr_char != '-' {
            break;
        }
    }
    count
}

fn nb_valid_rect(start: &Point, last_x: usize, board: &[&str], board_size: &Point) -> u32 {
    let mut count = 0;
    for y in start.1 + 1..board_size.1 {
        let left_char = board[y].chars().nth(start.0).unwrap();
        let right_char = board[y].chars().nth(last_x).unwrap();

        if left_char == '+' && right_char == '+' {
            if board[y][start.0..last_x]
                .chars()
                .all(|c| c == '-' || c == '+')
            {
                count += 1;
            }
        } else if left_char == ' ' || left_char == '-' || right_char == ' ' || right_char == '-' {
            break;
        }
    }
    count
}
