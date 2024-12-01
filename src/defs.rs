use std::{fs::File, io::{BufRead, Write}};

pub trait Solution {
    fn solve(&self, reader: &mut dyn BufRead, writer: &mut dyn Write);
}