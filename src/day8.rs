use std::{collections::HashMap, fs};

fn decode(v: &Vec<Vec<char>>, extended: bool) -> HashMap<Vec<char>, char> {
    let mut decoder = HashMap::new();
    let one = v.iter().find(|v| v.len() == 2).unwrap();
    decoder.insert(one.to_owned(), '1');
    let four = v.iter().find(|v| v.len() == 4).unwrap();
    decoder.insert(four.to_owned(), '4');
    let seven = v.iter().find(|v| v.len() == 3).unwrap();
    decoder.insert(seven.to_owned(), '7');
    let eight = v.iter().find(|v| v.len() == 7).unwrap();
    decoder.insert(eight.to_owned(), '8');
    if extended {
        let three = v
            .iter()
            .find(|v| v.len() == 5 && one.iter().all(|c| v.contains(c)))
            .unwrap();
        decoder.insert(three.to_owned(), '3');
        let two = v
            .iter()
            .find(|v| {
                v.len() == 5 && {
                    let filtered = v.iter().filter(|c| !four.contains(c) && !three.contains(c));
                    filtered.count() > 0
                }
            })
            .unwrap();
        decoder.insert(two.to_owned(), '2');
        let five = v
            .iter()
            .find(|v| v.len() == 5 && (*v != two) && (*v != three))
            .unwrap();
        decoder.insert(five.to_owned(), '5');
        let nine = v
            .iter()
            .find(|v| {
                v.len() == 6 && {
                    let filtered = v.iter().filter(|c| !four.contains(c));
                    filtered.count() == 2
                }
            })
            .unwrap();
        decoder.insert(nine.to_owned(), '9');
        let six = v
            .iter()
            .find(|v| {
                v.len() == 6 && {
                    let filtered = v.iter().filter(|c| !seven.contains(c));
                    filtered.count() == 4
                }
            })
            .unwrap();
        decoder.insert(six.to_owned(), '6');
        let zero = v
            .iter()
            .find(|v| v.len() == 6 && (*v != six) && (*v != nine))
            .unwrap();
        decoder.insert(zero.to_owned(), '0');
    }
    decoder
}

fn part1(vs: &Vec<(Vec<Vec<char>>, Vec<Vec<char>>)>) {
    let res: usize = vs
        .iter()
        .map(|v| {
            let decoder = decode(&v.0, false);
            v.1.iter().filter(|n| decoder.contains_key(*n)).count()
        })
        .sum();
    println!("{:?}", res)
}

fn part2(vs: &Vec<(Vec<Vec<char>>, Vec<Vec<char>>)>) {
    let res = vs
        .iter()
        .map(|v| {
            let decoder = decode(&v.0, true);
            let number = v.1.iter()
                .map(|n| decoder.get(n).unwrap().to_string())
                .collect::<Vec<_>>();
            number
                .join("")
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>();
    println!("{:?}", res)
}

pub fn run() {
    let data = fs::read_to_string("src/day8input.txt").unwrap();
    let parsed_data = data
        .lines()
        .map(|l| l.split(" | ").collect::<Vec<_>>())
        .map(|l| {
            let input = l[0]
                .split(" ")
                .map(|s| {
                    let mut svec = s.chars().collect::<Vec<_>>();
                    svec.sort();
                    svec
                })
                .collect::<Vec<_>>();
            let output = l[1]
                .split(" ")
                .map(|s| {
                    let mut svec = s.chars().collect::<Vec<_>>();
                    svec.sort();
                    svec
                })
                .collect::<Vec<_>>();
            (input, output)
        })
        .collect::<Vec<_>>();
    part1(&parsed_data);
    part2(&parsed_data);
}
