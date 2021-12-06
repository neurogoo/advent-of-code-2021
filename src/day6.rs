use std::fs;

fn simulation(fishes: &Vec<u32>, days: usize) {
    let mut colony: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0];
    for fish in fishes {
        colony[*fish as usize] += 1;
    }
    let mut new_fish: Vec<(u64, u64)> = Vec::new();
    for i in 0..days {
        let add_fish = colony[i % 7];
        let adult_fish: u64 = new_fish.iter().filter(|f| f.0 == 0).map(|f| f.1).sum();
        new_fish.retain(|f| f.0 != 0);
        for f in &mut new_fish {
            *f = (f.0 - 1, f.1);
        }
        new_fish.push((1, add_fish));
        colony[i % 7] += adult_fish;
    }
    println!(
        "{:?}",
        colony.iter().sum::<u64>() + new_fish.iter().map(|f| f.1).sum::<u64>()
    )
}

pub fn run() {
    let data = fs::read_to_string("src/day6input.txt").expect("");
    let parsed_data = data
        .split(",")
        .map(|s| s.trim().parse::<u32>().expect(""))
        .collect::<Vec<_>>();
    simulation(&parsed_data, 80);
    simulation(&parsed_data, 256);
}
