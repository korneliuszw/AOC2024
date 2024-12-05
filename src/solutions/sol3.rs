
use std::io::{BufRead, Write};
use regex::Regex;


use crate::defs::Solution;

pub struct Solution3 {}

impl Solution for Solution3 {

    fn solve(&self, reader: &mut dyn BufRead, writer: &mut dyn Write, part: u32) {
        let mut content = String::new();
        reader.read_to_string(&mut content).unwrap();
        if part == 2 {
            let re = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();
            let mut enable = true;
            let sum = re.captures_iter(&content).fold(0, |acc, val| {
                let full = val.get(0).unwrap().as_str();
                dbg!(&full);
                if full == "do()" {
                    enable = true;
                } else if full == "don't()" {
                    enable = false;
                } else if enable {
                    let a = val.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let b = val.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    return acc +  a * b;
                }
                acc
            });
            writeln!(writer, "{}", sum).unwrap();
        } else {
            let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
            let sum = re.captures_iter(&content).fold(0, |acc, val| {
                let a = val.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let b = val.get(2).unwrap().as_str().parse::<i32>().unwrap();
                return acc + a * b;
            });
            writeln!(writer, "{}", sum).unwrap();
        }
    }

}