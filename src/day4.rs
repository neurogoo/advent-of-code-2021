use std::fs;

fn board_won(board: &Vec<&str>) -> bool {
    let b = board.to_owned();
    let mut rows = Vec::new();
    for n in [0, 5, 10, 15, 20] {
        rows.push(b.get(n..n + 5).expect("").join(""));
    }
    let mut columns = Vec::new();
    for n in 0..5 {
        columns.push({
            let vals = (n..25)
                .step_by(5)
                .map(|i| *b.get(i).expect(""))
                .collect::<Vec<_>>();
            vals.join("")
        })
    }
    for row in rows {
        if row == "xxxxx" {
            return true;
        }
    }
    for column in columns {
        if column == "xxxxx" {
            return true;
        }
    }
    false
}

fn calculate_score(board: &Vec<&str>, last_number: i32) -> i32 {
    let unmarked: i32 = board
        .iter()
        .filter(|s| **s != "x")
        .map(|s| s.parse::<i32>().expect(""))
        .sum();
    unmarked * last_number
}

fn part1(bs: &Vec<Vec<&str>>, numbers: &Vec<&str>) {
    let mut boards = bs.clone();
    for number in numbers {
        for board in &mut boards {
            for s in board.iter_mut() {
                if s == number {
                    *s = "x"
                }
            }
            if board_won(&board) {
                println!(
                    "{}",
                    calculate_score(board, number.parse::<i32>().expect(""))
                );
                return;
            }
        }
    }
}

fn part2(bs: &Vec<Vec<&str>>, numbers: &Vec<&str>) {
    let mut boards = bs.clone();
    for number in numbers {
        for board in &mut boards {
            for s in board.iter_mut() {
                if s == number {
                    *s = "x"
                }
            }
        }
        let not_yet_won = boards.iter().filter(|b| !board_won(b)).collect::<Vec<_>>();
        if not_yet_won.len() == 0 {
            let won_boards = boards.iter().filter(|b| board_won(b)).collect::<Vec<_>>();
            println!(
                "{}",
                calculate_score(
                    won_boards.first().expect(""),
                    number.parse::<i32>().expect("")
                )
            );
            return;
        } else {
            boards.retain(|b| !board_won(b));
        }
    }
}

pub fn run() {
    let parsed_data = fs::read_to_string("src/day4input.txt")
        .expect("")
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let random_numbers = parsed_data
        .first()
        .expect("")
        .split(",")
        .collect::<Vec<_>>();
    let boards = &parsed_data[1..]
        .iter()
        .map(|board| {
            board
                .lines()
                .flat_map(|s| s.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    part1(boards, &random_numbers);
    part2(boards, &random_numbers);
}
