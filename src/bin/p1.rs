use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input/p1").expect("input/p1 doesn't exist");

    let values: Vec<_> = contents
        .split('\n')
        .map(|n| n.trim().parse::<i32>())
        .filter_map(|x| x.ok())
        .collect();

    // part 1
    println!("Part 1: {}", values.iter().fold(0, |acc, x| acc + x));

    // part 2
    let mut seen = HashSet::<i32>::new();
    let mut cur = 0;
    'outer: loop {
        for val in values.iter() {
            cur += val;
            if seen.contains(&cur) {
                println!("Part 2: {:?}", cur);
                break 'outer;
            } else {
                seen.insert(cur);
            }
        }
    }
}
