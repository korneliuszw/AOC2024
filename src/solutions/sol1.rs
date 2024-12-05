
use std::{collections::BinaryHeap, io::{BufRead, Write}};

use crate::defs::Solution;

pub struct Solution1 {}

impl Solution for Solution1 {
    fn solve(&self, reader: &mut dyn BufRead, writer: &mut dyn Write) {
        let mut map_a = BinaryHeap::<u32>::new();
        let mut map_b = BinaryHeap::<u32>::new();
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let values = line.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                    map_a.push(values[0]);
                    map_b.push(values[1]);
                }
                Err(_) => {
                    break;
                }
            }
        }
        let part_two = true;
        if part_two {
            let mut score = 0;
            while !map_a.is_empty() {
                let mut occurs = 0;
                let a = map_a.pop().unwrap();
                loop {
                    let b = *map_b.peek().unwrap();
                    if b < a {
                        break;
                    } else if b == a {
                        occurs += 1;
                    }
                    map_b.pop();
                }
                score += a * occurs;
            }
            writeln!(writer, "{}", score).unwrap();
        } else {
            let mut sum = 0;
            while !map_a.is_empty() {
                let dist = map_a.pop().unwrap().abs_diff(map_b.pop().unwrap());
                sum += dist;
            }
            writeln!(writer, "{}", sum).unwrap();
        }
    }
    
}
