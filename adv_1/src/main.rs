use std::{fs, fmt, num::ParseIntError, str::FromStr};
#[derive(Debug)]
struct Elf {
    calories: u32
}
impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Calories: {}",self.calories)
    }
}
impl std::str::FromStr for Elf {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut total: u32 = 0;
        for line in s.lines() {
            total += line.parse::<u32>().unwrap();
            
        }
        Ok(Elf {calories: total})
    }
}
impl Eq for Elf {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories
    }
}
impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.calories.partial_cmp(&other.calories)
    }
}
impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.calories.cmp(&other.calories)
    }
}
fn main() {
    let input = fs::read_to_string("/home/damian/rust/advofcode2022/adv_1/src/input.txt").expect("Can't open the file");
    let mut data : Vec<_>= input.split("\n\n").map(|b| Elf::from_str(b).unwrap()).collect();
    data.sort_by(|a,b| b.cmp(a));
    let a = data.iter().take(3).fold(0,|acc,elf| {acc+elf.calories});
    println!("Top calorie elf carries: {} calories",data.first().unwrap().calories);
    println!("Total calories from top carriers: {}",a);
}
