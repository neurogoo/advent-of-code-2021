use std::fs;

fn day1(vs: &[(String, i32)]) {
    let mut position = 0;
    let mut depth = 0;
    for v in vs {
        match (v.0.as_str(), v.1) {
            ("forward", amount) => position += amount,
            ("up", amount) => depth -= amount,
            ("down", amount) => depth += amount,
            _ => {}
        }
    }
    println!("{} {} {}", position, depth, position * depth);
}

fn day2(vs: &[(String, i32)]) {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for v in vs {
        match (v.0.as_str(), v.1) {
            ("forward", amount) => {
                position += amount;
                depth += aim * amount
            }
            ("up", amount) => aim -= amount,
            ("down", amount) => aim += amount,
            _ => {}
        }
    }
    println!("{} {} {}", position, depth, position * depth);
}

pub fn run() {
    let parsed_data: Vec<(String, i32)> = fs::read_to_string("src/day2input.txt")
        .expect("")
        .lines()
        .map(|s| s.split(' ').collect::<Vec<&str>>())
        .map(|s| (s[0].to_string(), s[1].parse::<i32>().expect("")))
        .collect();
    day1(&parsed_data);
    day2(&parsed_data);
}
