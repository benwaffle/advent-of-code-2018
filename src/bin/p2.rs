use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/p2").expect("input/p2 doesn't exist");
    let lines = contents.trim().split('\n');

    let mut two = 0;
    let mut three = 0;

    for line in lines.clone() {
        let mut chars = HashMap::<char, u32>::new();

        for c in line.chars() {
            let count = chars.entry(c).or_insert(0);
            *count += 1;
        }

        for (_, cnt) in &chars {
            if *cnt == 2 {
                two += 1;
                break;
            }
        }

        for (_, cnt) in &chars {
            if *cnt == 3 {
                three += 1;
                break;
            }
        }
    }

    println!("Part 1: {}", two * three);

    for line1 in lines.clone() {
        for line2 in lines.clone() {
            let mut diff = 0;
            let mut index: Option<usize> = None;
            assert_eq!(line1.len(), line2.len());

            for (i, c) in line1.chars().enumerate() {
                if line2.chars().nth(i).unwrap() != c {
                    diff += 1;
                    index = Some(i);
                }
            }

            index.map(|i| {
                if diff == 1 {
                    println!(
                        "Part 2: {}{}",
                        line1.get(..i).unwrap(),
                        line1.get((i + 1)..).unwrap()
                    );
                }
            });
        }
    }
}
