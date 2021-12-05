use std::fs;

fn zeros_more_than_ones(v: &[String], i: usize) -> bool {
    let numbers = v
        .iter()
        .map(|s| s.chars().nth(i).expect(""))
        .collect::<Vec<char>>();
    let zeros = numbers.iter().filter(|s| **s == '0').count();
    let ones = numbers.iter().filter(|s| **s == '1').count();
    zeros > ones
}

fn part1(v: &[String]) {
    let length = v.first().expect("").chars().count();
    let mut gamma_rate = Vec::new();
    let mut epsilon_rate = Vec::new();
    for i in 0..length {
        if zeros_more_than_ones(v, i) {
            gamma_rate.push("0");
            epsilon_rate.push("1");
        } else {
            gamma_rate.push("1");
            epsilon_rate.push("0");
        }
    }
    let gamma_rate2 = isize::from_str_radix(&gamma_rate.join(""), 2).expect("");
    let epsilon_rate2 = isize::from_str_radix(&epsilon_rate.join(""), 2).expect("");
    println!(
        "{} {} {}",
        gamma_rate2,
        epsilon_rate2,
        gamma_rate2 * epsilon_rate2
    );
}

fn part2(v: &[String]) {
    let length = v.first().expect("").chars().count();
    let mut oxygen_generator_rating: Vec<String> = v.to_owned();
    for i in 0..length {
        if oxygen_generator_rating.len() == 1 {
            break;
        }
        if zeros_more_than_ones(&oxygen_generator_rating, i) {
            oxygen_generator_rating = oxygen_generator_rating
                .iter()
                .filter(|s| s.chars().nth(i).expect("") == '0')
                .cloned()
                .collect();
        } else {
            oxygen_generator_rating = oxygen_generator_rating
                .iter()
                .filter(|s| s.chars().nth(i).expect("") == '1')
                .cloned()
                .collect();
        }
    }
    let mut c02_scrubber_rating: Vec<String> = v.to_owned();
    for i in 0..length {
        if c02_scrubber_rating.len() == 1 {
            break;
        }
        if zeros_more_than_ones(&c02_scrubber_rating, i) {
            c02_scrubber_rating = c02_scrubber_rating
                .iter()
                .filter(|s| s.chars().nth(i).expect("") == '1')
                .cloned()
                .collect();
        } else {
            c02_scrubber_rating = c02_scrubber_rating
                .iter()
                .filter(|s| s.chars().nth(i).expect("") == '0')
                .cloned()
                .collect();
        }
    }
    let ox2 = isize::from_str_radix(&oxygen_generator_rating.join(""), 2).expect("");
    let c022 = isize::from_str_radix(&c02_scrubber_rating.join(""), 2).expect("");
    println!("{} {} {}", ox2, c022, ox2 * c022);
}

#[allow(dead_code)]
pub fn run() {
    let parsed_data: Vec<_> = fs::read_to_string("src/day3input.txt")
        .expect("")
        .lines()
        .map(|s| s.to_string())
        .collect();
    part1(&parsed_data);
    part2(&parsed_data);
}
