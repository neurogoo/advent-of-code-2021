use std::{convert::identity, fs};

fn calculator<F>(subs: &Vec<i32>, cost_func: F)
where
    F: Fn(i32) -> i32,
{
    let min = subs.iter().min().expect("");
    let max = subs.iter().max().expect("");
    let min_pos: (i32, i32) = (*min..(max + 1))
        .map(|pos| {
            (
                pos,
                subs.iter().map(|sub| cost_func((sub - pos).abs())).sum(),
            )
        })
        .min_by(|val1: &(i32, i32), val2: &(i32, i32)| (val1.1).cmp(&val2.1))
        .unwrap();
    println!("{:?}", min_pos)
}

pub fn run() {
    let data = fs::read_to_string("src/day7input.txt").expect("");
    let parsed_data = data
        .split(",")
        .map(|s| s.trim().parse::<i32>().expect(""))
        .collect::<Vec<_>>();
    calculator(&parsed_data, identity);
    calculator(&parsed_data, |i| (1..(i + 1)).sum());
}
