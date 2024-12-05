

use std::{collections::HashMap, io::{BufRead, Write}, mem::swap};

use crate::defs::Solution;

pub struct Solution5 {}

impl Solution5 {
    fn inserting_dfs(&self, output: &mut Vec<u32>, graph: &mut HashMap<u32, (bool, Vec<u32>)>, element: u32) {
        if graph.get(&element).unwrap().0 {
            return;
        }
        graph.get_mut(&element).unwrap().0 = true;
        if graph.get(&element).is_some_and(|x| x.1.len() == 0) {
            output.push(element);
            return;
        }
        if let Some(neighbours) = graph.get(&element) {
            for neighbour in neighbours.1.clone() {
                self.inserting_dfs(output, graph, neighbour);
            }
            output.push(element);
        }
    }
    fn solve2(&self, map: HashMap<u32, Vec<u32>>, vec: impl Iterator<Item = Vec<u32>>) -> u32 {
        vec.fold(0, |acc, vals| {
            let mut graph : HashMap<u32, (bool, Vec<u32>)> = HashMap::new();
            let mut fixed = false;
            for (i, &el) in vals.iter().enumerate() {
                graph.entry(el).or_insert_with(|| (false, Vec::new()));
                if let Some(others) = map.get(&el) {
                    for &other in others {
                        for (j, &prev_el) in vals.iter().enumerate() {
                            if prev_el == other {
                                graph.entry(el).or_insert_with(|| (false, Vec::new())).1.push(other);
                                if j > i {
                                    fixed = true;
                                }
                                break;
                            }
                        }
                    }
                }
            }
            if !fixed {
                return acc;
            }
            let mut output = Vec::new();
            for &el in &vals {
                self.inserting_dfs(&mut output, &mut graph, el);
            }
            return acc + output[output.len() / 2];
        })
    }

}

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
        let it = reader.lines().filter_map(|line| {
            Some(line.ok()?.trim().split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>())
        });
        let sum = if part == 2 {
            self.solve2(map, it)
        } else {
            it.fold(0, |acc, vals| {
                for (i, &el) in vals.iter().enumerate() {
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
        })
    };
    writeln!(writer, "{}", sum).unwrap();
    }

}