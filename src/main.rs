use std::{io::{BufReader, BufRead}, fs::File};
use nalgebra::DMatrix;

const STONE: char = 'O';
const BOX: char = '#';

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

fn shift_grid(grid: &mut DMatrix<char>) -> DMatrix<char> {
    grid.to_owned()
}

fn sum_stones(grid: &DMatrix<char>) -> i32 {
    let mut sum = 0;
    for i in 0..grid.nrows() {
        for j in 0..grid.ncols() {
            if grid[(i, j)] == STONE {
                sum += 10 - i;
            }
        }
    }
    sum as i32
}

fn solution(filename: &str) -> i32 {
    let mut grid = get_grid(filename);
    grid = shift_grid(&mut grid);
    sum_stones(&grid)
}

fn main() {
    assert_eq!(solution("example.txt"), 136);
    assert_eq!(solution("input.txt"), 0);
}
