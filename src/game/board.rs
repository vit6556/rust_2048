use std::io;
use std::io::Write;

pub struct Board {
    size: usize,
    max_value: usize,
    board: Vec<Vec<usize>>,
}