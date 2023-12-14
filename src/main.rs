use std::{io::{BufReader, BufRead}, fs::File};
use nalgebra::{DMatrix, RowDVector};

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
                cloned_grid[(i + row_index, j)] = *items[i];
            }
            row_index = indices_and_tiles.last().unwrap_or(&(0, &BOX)).0 + row_index + 1;
        }
    }
    cloned_grid.to_owned()
}

fn sum_stones(grid: &DMatrix<char>) -> i32 {
    let mut sum = 0;
    let nrows = grid.nrows();
    for i in 0..nrows {
        for j in 0..grid.ncols() {
            if grid[(i, j)] == STONE {
                sum += nrows - i;
            }
        }
    }
    sum as i32
}

fn rotate_grid(grid: &mut DMatrix<char>) -> &mut DMatrix<char> {
    grid.transpose_mut();
    for i in 0..grid.nrows() {
        let row_reversed = grid.row_mut(i).iter().rev().map(|x| x.to_owned()).collect::<Vec<char>>();
        grid.set_row(i, &RowDVector::from_row_slice(&row_reversed));
    }
    grid
}

fn solution(filename: &str) -> i32 {
    let mut grid = get_grid(filename);
    grid = shift_grid(&mut grid);
    sum_stones(&grid)
}

fn solution_2(filename: &str, cycles: u32) -> i32 {
    let mut grid = get_grid(filename);
    for _ in 0..cycles {
        for _ in 0..4 {
            //println!("part {}", i);
            grid = shift_grid(&mut grid);
            //println!("shifted: {}", grid);
            rotate_grid(&mut grid);
            //println!("rotated: {}", grid);
        }
    }
    sum_stones(&grid)
}


fn main() {
    assert_eq!(solution("example.txt"), 136);
    assert_eq!(solution("input.txt"), 105623);
    assert_eq!(solution_2("example.txt", 1000000000), 64);
    assert_eq!(solution_2("input.txt", 1000000000), 0);
}
