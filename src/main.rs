use std::{io::{BufReader, BufRead}, fs::File};
use nalgebra::DMatrix;

fn get_grid(filename: &str) -> DMatrix<char> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut grid = Vec::new();
    let mut nrows = 0;
    for line in BufReader::lines(reader) {
        let line = line.unwrap();
        grid.extend(line.chars());
        nrows += 1;
    }
    DMatrix::from_row_slice(nrows, grid.len() / nrows, &grid)
}

fn solution(filename: &str) -> i32 {
    let grid = get_grid(filename);
    println!("Part 1: {}", grid);
    0
}

fn main() {
    assert_eq!(solution("example.txt"), 136);
    assert_eq!(solution("input.txt"), 0);
}
