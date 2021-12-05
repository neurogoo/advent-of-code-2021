use std::{cmp::max, cmp::min, collections::HashMap, fs};

fn part12(v: &Vec<Vec<Vec<u32>>>, part2: bool) {
    let coords = v
        .iter()
        .flat_map(|v| {
            if v[0][0] == v[1][0] {
                (min(v[0][1], v[1][1])..(max(v[0][1], v[1][1]) + 1))
                    .map(|val| (v[0][0], val))
                    .collect::<Vec<_>>()
            } else if v[0][1] == v[1][1] {
                (min(v[0][0], v[1][0])..(max(v[0][0], v[1][0]) + 1))
                    .map(|val| (val, v[0][1]))
                    .collect::<Vec<_>>()
            } else {
                if part2 {
                    let x_range = if v[0][0] < v[1][0] {
                        (v[0][0]..(v[1][0] + 1)).collect::<Vec<_>>()
                    } else {
                        (v[1][0]..(v[0][0] + 1)).rev().collect::<Vec<_>>()
                    };
                    let y_range = if v[0][1] < v[1][1] {
                        (v[0][1]..(v[1][1] + 1)).collect::<Vec<_>>()
                    } else {
                        (v[1][1]..(v[0][1] + 1)).rev().collect::<Vec<_>>()
                    };
                    x_range
                        .iter()
                        .zip(y_range.iter())
                        .map(|x| (*x.0, *x.1))
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                }
            }
        })
        .collect::<Vec<_>>();
    let mut points = HashMap::new();
    for coord in coords {
        let count = points.entry(coord).or_insert(0);
        *count += 1;
    }
    let val = points.values().filter(|v| **v >= 2).count();
    println!("{:?}", val);
}

pub fn run() {
    let data = fs::read_to_string("src/day5input.txt").expect("");
    let parsed_data = data
        .lines()
        .map(|s| {
            s.split(" -> ")
                .map(|s| {
                    s.split(",")
                        .map(|s| s.parse::<u32>().expect(""))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part12(&parsed_data, false);
    part12(&parsed_data, true);
}
