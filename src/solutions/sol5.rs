

use std::{collections::HashMap, io::{BufRead, Write}};

use crate::defs::Solution;

pub struct Solution5 {}

impl Solution for Solution5 {

    fn solve(&self, reader: &mut dyn BufRead, writer: &mut dyn Write, part: u32) {
        let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
        for line in reader.lines() {
            let binding = line.unwrap();
            if binding.len() == 0 {
                break;
            }
            let arr = binding.trim().split('|').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            if map.get(&arr[1]).is_some() {
                let vec = map.get_mut(&arr[1]).unwrap();
                vec.push(arr[0]);
            } else {
                map.insert(arr[1], vec![arr[0]]);
            }
        }
        let sum = reader.lines().filter_map(|line| {
            Some(line.ok()?.trim().split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>())
        }).fold(0, |acc, vals| {
            'outer: for (i, &el) in vals.iter().enumerate() {
                if let Some(others) = map.get(&el) {
                    for &other in others {
                        for (j, &prev_el) in vals.iter().enumerate() {
                            if prev_el == other {
                                if j > i {
                                    return acc;
                                }
                                break;
                            }
                        }
                    }
                }
            }
            return acc + vals[vals.len() / 2];
        });
        writeln!(writer, "{}", sum).unwrap();
    }

}