use anyhow::Result;
use std::{collections::HashSet, fs};

fn main() -> Result<()> {
    const START_WINDOW_SIZE: usize = 4;
    const MESSAGE_WINDOW_SIZE: usize = 14;
    let input = fs::read_to_string("/home/damian/rust/advofcode2022/adv_6/input.txt")?
        .chars()
        .collect::<Vec<char>>();

    for (counter, window) in input.windows(START_WINDOW_SIZE).enumerate() {
        let a: HashSet<&char> = HashSet::from_iter(window);
        if a.len() == START_WINDOW_SIZE {
            /*if all were different then set would equal widow size*/
            println!("part1: {}", counter + MESSAGE_WINDOW_SIZE);
            break;
        }
    }

    for (counter, window) in input.windows(MESSAGE_WINDOW_SIZE).enumerate() {
        let a: HashSet<&char> = HashSet::from_iter(window);
        if a.len() == MESSAGE_WINDOW_SIZE {
            println!("part2: {}", counter + MESSAGE_WINDOW_SIZE);
            break;
        }
    }

    Ok(())
}
