use std::fs;

enum LineStatus {
    Correct,
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn process_line(s: &String) -> LineStatus {
    let mut queue = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => queue.push(c),
            _ => {
                let previous_char = queue.pop().unwrap();
                let right_char = match previous_char {
                    '(' => c == ')',
                    '[' => c == ']',
                    '{' => c == '}',
                    '<' => c == '>',
                    _ => false,
                };
                if !right_char {
                    return LineStatus::Corrupted(c);
                }
            }
        }
    }
    if !queue.is_empty() {
        LineStatus::Incomplete(queue)
    } else {
        LineStatus::Correct
    }
}

fn part1(v: &Vec<String>) {
    let res = v
        .iter()
        .filter_map(|s| match process_line(s) {
            LineStatus::Corrupted(')') => Some(3),
            LineStatus::Corrupted(']') => Some(57),
            LineStatus::Corrupted('}') => Some(1197),
            LineStatus::Corrupted('>') => Some(25137),
            _ => None,
        })
        .sum::<u32>();
    println!("{:?}", res);
}

fn part2(v: &Vec<String>) {
    let mut res = v
        .iter()
        .filter_map(|s| match process_line(s) {
            LineStatus::Incomplete(queue) => Some(
                queue
                    .iter()
                    .rev()
                    .map(|c| match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    })
                    .fold(0 as u64, |res, score| res * 5 + score),
            ),
            _ => None,
        })
        .collect::<Vec<_>>();
    res.sort();
    println!("{:?}", res[(res.len() as f32 / 2.0).ceil() as usize - 1]);
}

#[allow(dead_code)]
pub fn run() {
    let data = fs::read_to_string("src/day10input.txt").unwrap();
    let parsed_data = data.lines().map(|s| s.to_string()).collect::<Vec<_>>();
    part1(&parsed_data);
    part2(&parsed_data);
}
