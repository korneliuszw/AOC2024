
use std::io::{BufRead, Write};

use crate::defs::Solution;

pub struct Solution2 {}

impl Solution for Solution2 {

    fn solve(&self, reader: &mut dyn BufRead, writer: &mut dyn Write) {
        let part_two = true;
        let sum = reader.lines().filter(|v| v.is_ok()).fold(0, |acc, line| {
            let binding = line.unwrap();
            let arr = binding.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            let mut desc = 0;
            let mut tolerated = 0;
            for (a, b) in arr.iter().zip(arr.iter().skip(1)) {
                if a == b || a.abs_diff(*b) > 3 || (a < b && desc == -1) || (a > b && desc == 1) {
                    tolerated += 1;
                    if tolerated > 1 || !part_two {
                        return acc;
                    }
                } else if desc == 0 {
                    desc = if a < b { 1 } else { -1 };
                }
            }
            return acc + 1;
        });
        writeln!(writer, "{}", sum).unwrap();
    }

}