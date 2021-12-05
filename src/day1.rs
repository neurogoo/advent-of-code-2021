use std::fs;

fn part1(input: &Vec<i32>) -> () {
    let data: i32 = input
        .windows(2)
        .map(|x| if x[0] < x[1] { 1 } else { 0 })
        .sum();
    println!("{:?}", data);
}

fn part2(input: &Vec<i32>) -> () {
    let data: Vec<i32> = input.windows(3).map(|x| x.iter().sum()).collect();
    let data2: i32 = data
        .windows(2)
        .map(|x| if x[0] < x[1] { 1 } else { 0 })
        .sum();
    println!("{:?}", data2);
}

#[allow(dead_code)]
pub fn run() -> () {
    let parsed_data = fs::read_to_string("src/day1input.txt")
        .expect("")
        .lines()
        .map(|s| s.parse::<i32>().expect(""))
        .collect::<Vec<i32>>();
    part1(&parsed_data);
    part2(&parsed_data);
}
