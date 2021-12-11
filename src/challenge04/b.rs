type Board = Vec<Vec<(i32, bool)>>;

pub fn run(stdin: String) -> String {
    let mut splits = stdin.split("\n\n");

    let draw_pool: Vec<i32> = splits
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = splits.map(map_board).collect();

    if let Some((number, winning)) = play(draw_pool, &mut boards) {
        return caclulate_score(number, winning).to_string();
    }

    "".to_string()
}

fn caclulate_score(number: i32, board: Board) -> i32 {
    let mut sum = 0;
    for col in board {
        for row in col {
            if !row.1 {
                sum += row.0
            }
        }
    }

    sum * number
}

fn play<'a>(draw_pool: Vec<i32>, boards: &mut Vec<Board>) -> Option<(i32, Board)> {
    for number in draw_pool {
        let len = boards.len();
        for board in &mut *boards {
            play_number(number, board);
            if len == 1 && has_won(&board) {
                return Some((number, board.clone()));
            }
        }

        let mut i = 0;
        while i < boards.len() {
            if has_won(&boards[i]) {
                boards.remove(i);
            } else {
                i += 1;
            }
        }
    }

    return None;
}

fn play_number(number: i32, board: &mut Board) {
    for col in board {
        for cell in col {
            if cell.0 == number {
                cell.1 = true;
            }
        }
    }
}

fn has_won(board: &Board) -> bool {
    for column in 0..5 {
        let mut all = true;
        for row in 0..5 {
            if !board[column][row].1 {
                all = false
            }
        }
        if all {
            return true;
        }
    }

    for row in 0..5 {
        let mut all = true;
        for column in 0..5 {
            if !board[column][row].1 {
                all = false
            }
        }
        if all {
            return true;
        }
    }

    return false;
}

fn map_board(s: &str) -> Vec<Vec<(i32, bool)>> {
    let spaces = regex::Regex::new(r"\s+").unwrap();

    s.split('\n')
        .map(|line| {
            spaces
                .split(line)
                .filter(|cell| !cell.is_empty())
                .map(|cell| cell.parse::<i32>().unwrap())
                .map(|cell| (cell, false))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::util::testing_utils::read_test_resource;

    #[test]
    fn run() {
        assert_eq!("55770", super::run(read_test_resource("day04.txt")));
    }
}
