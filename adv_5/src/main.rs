use std::{str::FromStr, num::ParseIntError, fs};
use itertools::Itertools;
use regex::Regex;
type Stack = Vec<char>;

#[derive(Debug)]
struct Ship {
    stacks: Vec<Stack>
}

impl FromStr for Ship {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('\n').rev();
        let columns = iter.next().unwrap().trim().split(' ').last().unwrap().parse::<usize>().unwrap();
        let mut stacks: Vec<Stack> = Vec::new();
        stacks.resize(columns,Stack::new());
        for line in iter {
            for c in 0..columns {
                let crat = line.as_bytes()[1+c*4] as char;
                if crat != ' ' {
                 stacks[c].push(crat);
                }
            }
        }
        Ok(Ship {stacks})
    }
}
fn crane_move_9000(ship: &mut Ship, mv: &Move) {
    for _ in 0..mv.amount {
        let temp = ship.stacks[mv.from-1].pop().unwrap();
        ship.stacks[mv.to-1].push(temp);
    }
}
fn crane_move_9001(ship: &mut Ship, mv: &Move) {
    
    let len = ship.stacks[mv.from-1].len();
    let temp = ship.stacks[mv.from-1].drain(len-mv.amount..len).collect_vec();
    ship.stacks[mv.to-1].extend(temp.iter());
}
#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize
}
impl FromStr for Move {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let captures = re.captures(s).unwrap();
        Ok (
            Move
            {
                amount: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                from: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                to: captures.get(3).unwrap().as_str().parse::<usize>().unwrap() 
            })
    }
}

fn main() {
    let input = fs::read_to_string("/home/damian/rust/advofcode2022/adv_5/input.txt").unwrap();
    let b = input
        .split("\n\n")
        .collect_vec();
    let mut ship = Ship::from_str(b[0]).unwrap();
    let moves = b[1].trim().split('\n').map(|f| Move::from_str(f).unwrap()).collect_vec();
    //moves.iter().for_each(|m| crane_move_9000(&mut ship,&m));
    moves.iter().for_each(|m| crane_move_9001(&mut ship,&m));
    ship.stacks.iter().for_each(|s| print!("{}",s.last().unwrap_or(&' ')));
    println!();
    
}
