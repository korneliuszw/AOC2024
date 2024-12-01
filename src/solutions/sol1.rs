
use std::{collections::BinaryHeap, io::{BufRead, Write}};

use crate::defs::Solution;

pub struct Solution1 {}

impl Solution for Solution1 {
    fn solve(&self, reader: &mut dyn BufRead, writer: &mut dyn Write) {
        let mut mapA = BinaryHeap::<u32>::new();
        let mut mapB = BinaryHeap::<u32>::new();
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let values = line.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                    mapA.push(values[0]);
                    mapB.push(values[1]);
                }
                Err(_) => {
                    break;
                }
            }
        }
        let mut sum = 0;
        while !mapA.is_empty() {
            let dist = mapA.pop().unwrap().abs_diff(mapB.pop().unwrap());
            sum += dist;
        }
        writeln!(writer, "{}", sum).unwrap();
    }
    
}
