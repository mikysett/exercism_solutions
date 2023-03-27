struct Point(usize, usize);

pub fn count(board: &[&str]) -> u32 {
    let board_size = Point(board.first().map(|row| row.len()).unwrap_or(0), board.len());

    board
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c == '+')
                .map(|(x, _)| count_rect_from_point(&Point(x, y), board, &board_size))
                .sum::<u32>()
        })
        .sum()
}

fn count_rect_from_point(start: &Point, board: &[&str], board_size: &Point) -> u32 {
    board[start.1][start.0 + 1..board_size.0]
        .chars()
        .enumerate()
        .map(|(x, c)| (x + start.0 + 1, c))
        .take_while(|(_, c)| *c == '-' || *c == '+')
        .filter(|(_, c)| *c == '+')
        .fold(0, |acc, (x, _)| {
            acc + nb_valid_rect(start, x, board, board_size)
        })
}

fn nb_valid_rect(start: &Point, last_x: usize, board: &[&str], board_size: &Point) -> u32 {
    board[start.1 + 1..board_size.1]
        .iter()
        .map(|row| {
            (
                row,
                row.chars().nth(start.0).unwrap(),
                row.chars().nth(last_x).unwrap(),
            )
        })
        .take_while(|(_line, left_c, right_c)| {
            *left_c != ' ' && *left_c != '-' && *right_c != ' ' && *right_c != '-'
        })
        .filter(|(row, left_c, right_c)| {
            *left_c == '+'
                && *right_c == '+'
                && row[start.0..last_x].chars().all(|c| c == '-' || c == '+')
        })
        .count() as u32
}
