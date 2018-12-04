use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/p1").expect("input/p1 doesn't exist");

    let values: Vec<_> = contents
        .split('\n')
        .map(|n| n.trim().parse::<i32>())
        .filter_map(|x| x.ok())
        .collect();

    // part 1
    println!("Part 1: {}", values.iter().sum::<i32>());

    // part 2
    let mut seen = HashSet::<i32>::new();
    let mut cur = 0;
    for val in values.iter().cycle() {
        cur += val;
        if seen.contains(&cur) {
            println!("Part 2: {:?}", cur);
            break;
        } else {
            seen.insert(cur);
        }
    }
}
