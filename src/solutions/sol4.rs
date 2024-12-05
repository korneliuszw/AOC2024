
use std::io::{BufRead, Write};

use crate::defs::Solution;

pub struct Solution4 {
}

impl Solution4 {
    fn search(&self, vec: &Vec<Vec<char>>, ox: isize, oy: isize, step_x: isize, step_y: isize, word: &str) -> bool {
        let mut x = ox;
        let mut y = oy;
        let mut word_arr = word.chars().peekable();
        let max_y = vec.len() as isize;
        if y < 0 || y >= max_y {
            return false;
        }
        let max_x = vec[y as usize].len() as isize;
        while x >= 0 && x < max_x && y >= 0 && y < max_y &&*word_arr.peek().unwrap() == vec[y as usize][x as usize] {
            word_arr.next();
            if word_arr.peek().is_none() {
                println!("ox: {} oy:{} x:{} y:{} dx:{} dy:{}", ox, oy, x, y, step_x, step_y);
                return true
            }
            x += step_x;
            y += step_y;
        }
        false
    }
}

impl Solution for Solution4 {
    
    fn solve(&self, reader: &mut dyn BufRead, writer: &mut dyn Write, part: u32) {
        let char_array = reader.lines().filter_map(|v| {
            Some(v.ok()?.chars().collect::<Vec<char>>())
        }).collect::<Vec<Vec<char>>>();
        let mut sum = 0;
        for y in 0..char_array.len() as isize {
            for x in 0..char_array[y as usize].len() as isize {
                if part == 2 {
                    if char_array[y as usize][x as usize] == 'A' {
                        if self.search(&char_array, x - 1 as isize, y - 1, 1, 1, "MAS") || self.search(&char_array, x - 1, y -1, 1, 1, "SAM") {
                            if self.search(&char_array, x + 1, y - 1, -1, 1, "SAM") || self.search(&char_array, x + 1, y - 1, -1, 1, "MAS") {
                                sum += 1;
                            }
                        }
                    }
                } else {
                    if char_array[y as usize][x as usize] == 'X' {
                        if self.search(&char_array, x as isize, y as isize,  -1, 0, "XMAS") { sum += 1; }
                        if self.search(&char_array, x as isize, y as isize,  1, 0, "XMAS") { sum += 1;}
                        if self.search(&char_array, x as isize, y as isize,  0, 1, "XMAS") { sum += 1;}
                        if self.search(&char_array, x as isize, y as isize,  0, -1, "XMAS") { sum += 1;}
                        if self.search(&char_array, x as isize, y as isize,  -1, 1, "XMAS") { sum += 1;}
                        if self.search(&char_array, x as isize, y as isize,  1, 1, "XMAS") { sum += 1;}
                        if self.search(&char_array, x as isize, y as isize,  -1, -1, "XMAS") { sum += 1;}
                        if self.search(&char_array, x as isize, y as isize,  1, -1, "XMAS") { sum += 1;}
                    }
                }
            }
        }
        writeln!(writer, "{}", sum).unwrap();
    }

}