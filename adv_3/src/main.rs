#![feature(array_chunks)]
use std::{fs, str::FromStr, string::ParseError};
#[derive(Debug)]
struct Rucksack {
    compartment1: String,
    compartment2: String,
}
impl Rucksack {
    fn compose(&self) -> String {
        return self.compartment1.clone() + &self.compartment2;
    }
    fn find_common(&self) -> Option<char> {
        let mut shorter = &self.compartment2;
        let mut longer = &self.compartment1;
        if self.compartment1.len() < self.compartment2.len() {
            shorter = &self.compartment1;
            longer = &self.compartment2;
        }
        for i in 0..shorter.len() {
            for j in 0..longer.len() {
                if shorter.as_bytes()[i] == longer.as_bytes()[j] {
                    return Some(shorter.as_bytes()[i] as char);
                }
            }
        }
        return None;
    }
}
impl FromStr for Rucksack {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rucksack {
            compartment1: s[0..s.len() / 2].to_string(),
            compartment2: s[s.len() / 2..s.len()].to_string(),
        })
    }
}
fn find_common_badge(r_slice: &[Rucksack]) -> Option<char> {
    /*we could make this one a tad bit cleaner but not much time today <sad_face>*/
    for i in 'a'..='z' {
        let mut k = 0;
        for r in r_slice {
            k += r.compose().contains(i) as u32;
        }
        if k >= 3 {
            return Some(i);
        }
    }
    for j in 'A'..='Z' {
        let mut k = 0;
        for r in r_slice {
            k += r.compose().contains(j) as u32;
        }
        if k >= 3 {
            return Some(j);
        }
    }
    None
}
fn to_priority(c: char) -> u32 {
    let a: u32 = c as u32;
    if a >= 97 {
        a - 96
    } else {
        a - 65 + 27
    }
}
fn main() {
    let input = fs::read_to_string("/home/damian/rust/advofcode2022/adv_3/input.txt")
        .expect("Can't open the file");
    let rucksacks: Vec<_> = input
        .split("\n")
        .filter(|a| a.len() > 0)
        .map(|line| Rucksack::from_str(line).unwrap())
        .collect();
    let sum_part1: u32 = rucksacks
        .iter()
        .map(|f| f.find_common().unwrap())
        .map(|f| to_priority(f))
        .sum();
    println!("Part 1 sum :{}", sum_part1);
    let sum_part2: u32 = rucksacks
        .array_chunks::<3>()
        .map(|i| find_common_badge(i).unwrap())
        .map(|f| to_priority(f))
        .sum();
    println!("Part 2 sum: {}", sum_part2);
}
