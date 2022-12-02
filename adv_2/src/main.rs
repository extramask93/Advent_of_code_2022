use std::{str::FromStr, string::ParseError, fs};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug)]
struct ParseEnumError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PlayOutcome {
    Lost = 0,
    Draw = 3,
    Win = 6,
}
impl FromStr for PlayOutcome {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = s.trim();
        match ss {
            "X" => Ok(PlayOutcome::Lost),
            "Y" => Ok(PlayOutcome::Draw),
            "Z" => Ok(PlayOutcome::Win),
            _ => Err(ParseEnumError)
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl Shape {
    fn match_against(&self, other: &Shape) -> PlayOutcome {
        match (self,other)
        {
            (Shape::Rock,Shape::Scissors) => PlayOutcome::Win,
            (Shape::Paper,Shape::Rock)  => PlayOutcome::Win,
            (Shape::Scissors,Shape::Paper) => PlayOutcome::Win,
            (a,b) if a == b => PlayOutcome::Draw,
            (_,_) => PlayOutcome::Lost
        }
    }
    fn generate_move(&self, exp: PlayOutcome) -> Shape {
        match (self,exp) {
            (a, PlayOutcome::Draw) => *a,
            (a, PlayOutcome::Win) => Shape::try_from(((((*a as u32) - 1 )+1) % 3)+1).unwrap(),
            (a, PlayOutcome::Lost) => Shape::try_from(((((*a as u32) - 1 )+3-1) % 3)+1).unwrap()
        }
    }
    
}
impl FromStr for Shape {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = s.trim();
        match ss {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(ParseEnumError)
        }
    }
}
#[derive(Debug)]
struct Round {
    player1_shape: Shape,
    player2_shape: Shape,
    result: PlayOutcome,
    score: u32
}
impl Round {
    fn new(s1:Shape,s2:Shape) -> Self{
        let sc  =  s2.match_against(&s1) as u32 + s2 as u32;
        Round{player1_shape:s1, player2_shape: s2,result: s2.match_against(&s1), score:sc}
    }
    fn new_2(s1:Shape,r: PlayOutcome) -> Self{
        let s2 = s1.generate_move(r);
        let sc  =  s2.match_against(&s1) as u32 + s2 as u32;
        Round{player1_shape:s1, player2_shape: s2,result: s2.match_against(&s1), score:sc}
    }
}
impl FromStr for Round {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items :Vec<_>= s.trim().split(" ").collect();
        assert!(items.len() == 2);
        let shape1 = items[0].parse::<Shape>().unwrap();
        let shape2 = items[1].parse::<PlayOutcome>().unwrap();
        Ok(Round::new_2(shape1,shape2))
        /*part 1*/
        //Ok(Round::new(shape1,shape2))

    }
}
fn main() {
    let input = fs::read_to_string("/home/damian/rust/advofcode2022/adv_2/input.txt").expect("Can't open the file");
    let score = input.split("\n").filter(|a| a.len()>0)
        .map(|b| Round::from_str(b).unwrap())
        .fold(0,|acc,r| {acc+r.score});
    println!("{}",score);
}
