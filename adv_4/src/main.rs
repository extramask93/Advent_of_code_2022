use itertools::Itertools;
use std::{collections::HashSet, fs, num::ParseIntError, str::FromStr, string::ParseError};

#[derive(Debug)]
struct CleaningSection {
    section: HashSet<u32>,
}
impl FromStr for CleaningSection {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (st, en) = s
            .trim()
            .split('-')
            .map(|c| c.parse::<u32>().unwrap())
            .next_tuple()
            .unwrap();
        Ok(CleaningSection {
            section: HashSet::from_iter(st..=en),
        })
    }
}
#[derive(Debug)]
struct CleaningPair {
    p1: CleaningSection,
    p2: CleaningSection,
}
impl CleaningPair {
    fn is_fully_overlaping(&self) -> bool {
        self.p1.section.is_subset(&self.p2.section) || self.p2.section.is_subset(&self.p1.section)
    }
    fn is_partially_overlapping(&self) -> bool {
        !self.p1.section.is_disjoint(&self.p2.section)
    }
}
impl FromStr for CleaningPair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .trim()
            .split(',')
            .map(|cc| CleaningSection::from_str(cc).unwrap())
            .next_tuple()
            .unwrap();
        Ok(CleaningPair { p1: a, p2: b })
    }
}
fn main() {
    let input = fs::read_to_string("/home/damian/rust/advofcode2022/adv_4/input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|l| CleaningPair::from_str(l).unwrap())
        .collect_vec();
    let answer1 = input.iter().filter(|cp| cp.is_fully_overlaping()).count();
    println!("# of pairs fully contained in other: {answer1}");
    let answer2 = input
        .iter()
        .filter(|cp| cp.is_partially_overlapping())
        .count();
    println!("# of pairs partially contained in other: {answer2}");
}
