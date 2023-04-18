use std::str;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut field: Vec<Vec<u8>> = minefield
        .iter()
        .map(|line| line.as_bytes().to_owned())
        .collect();

    let x_max = field.first().unwrap_or(&vec![]).len();
    let y_max = field.len();

    minefield.iter().enumerate().for_each(|(y, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == b'*')
            .for_each(|(x, _)| {
                get_neighbors(x, y, x_max, y_max).iter().for_each(|(x, y)| {
                    let curr_val = field[*y][*x];

                    if curr_val != b'*' {
                        if curr_val == b' ' {
                            field[*y][*x] = b'1';
                        } else {
                            field[*y][*x] = curr_val + 1;
                        }
                    }
                })
            })
    });

    field
        .into_iter()
        .map(|line| unsafe { String::from_utf8_unchecked(line) })
        .collect()
}

fn get_neighbors(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let mut possible_x = vec![x];
    let mut possible_y = vec![y];
    if x != 0 {
        possible_x.push(x - 1);
    }
    if x < x_max - 1 {
        possible_x.push(x + 1);
    }
    if y != 0 {
        possible_y.push(y - 1);
    }
    if y < y_max - 1 {
        possible_y.push(y + 1);
    }

    possible_x
        .iter()
        .flat_map(|curr_x| {
            possible_y
                .iter()
                .map(|curr_y| (curr_x, curr_y))
                .filter(|(&curr_x, &curr_y)| curr_x != x || curr_y != y)
                .map(|v| (*v.0, *v.1))
                .collect::<Vec<_>>()
        })
        .collect()
}
