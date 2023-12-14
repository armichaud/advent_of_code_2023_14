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
    let mut cloned_grid = grid.clone();
    for j in 0..grid.ncols() {
        let mut row_index = 0;
        let col = grid.column(j);
        while row_index < col.len() {
            let indices_and_tiles = col.iter().skip(row_index).enumerate().take_while(|(_, &c)| c != BOX).collect::<Vec<(usize, &char)>>();
            row_index = indices_and_tiles.last().unwrap_or(&(row_index, &BOX)).0 + 1;
            let mut items = indices_and_tiles.iter().map(|(_, c)| *c).collect::<Vec<&char>>();
            items.sort_by(|&&a, &&b| {
                if a == STONE && b == STONE {
                    std::cmp::Ordering::Equal
                } else if a == STONE {
                    std::cmp::Ordering::Less
                } else if b == STONE {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            for i in 0..items.len() {
                cloned_grid[(i, j)] = *items[i];
            }
        }
    }
    println!("{}", cloned_grid);
    cloned_grid.to_owned()
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
