use std::io::{BufRead, Write};

pub trait Solution {
    fn solve(&self, reader: &mut dyn BufRead, writer: &mut dyn Write, part : u32);
}