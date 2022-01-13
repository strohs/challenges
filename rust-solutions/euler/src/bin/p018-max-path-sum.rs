use std::{cmp, io};
use std::fs::File;
use std::path::Path;
use std::io::BufRead;

/// # Maximum Path Sum I
/// By starting at the top of the triangle below and moving to adjacent numbers on the row below,
/// the maximum total from top to bottom is 23.
/// ```
///    3
///   7 4
///  2 4 6
/// 8 5 9 3
/// ```
/// That is, 3 + 7 + 4 + 9 = 23.
/// The input for this challenge is in the file: p18-input.txt
/// ## Solution Details:
/// Rather than starting from the root and trying to brute force all possible paths to the bottom
/// of the tree, this solution will start from the bottom row - 1, compute the max sum for each
/// node, and store the max sum in that node. As the algorithm works towards the root, each node
/// will contain the max sum of the nodes below it. The root node will contain the maximum sum path
/// once the algorithm is finished.

/// read a file line by line. returns an Iterator to the file's lines
/// fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
///     where P: AsRef<Path>, {
///     let file = File::open(file_path)?;
///     Ok(io::BufReader::new(file).lines())
/// }

/// read a file of space separated integers into a 2D Vector
fn file_to_matrix<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<i32>>, io::Error> {
    let file = File::open(path)?;
    let mut matrix = vec![];
    for buff_line in io::BufReader::new(file).lines() {
        if let Ok(line) = buff_line {
            let nums = line.split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            matrix.push(nums);
        }
    }
    Ok(matrix)
}

fn print_matrix(m: &Vec<Vec<i32>>) {
    let max_width = m.last().unwrap().len();
    for row_idx in 0..m.len() {
        let row_pad = ((max_width - m[row_idx].len()) * 5) / 2;
        print!("{:>pad$}", " ", pad = row_pad);
        for col_idx in 0..m[row_idx].len() {
            print!(" {:04}", m[row_idx][col_idx]);
        }
        println!();
    }
}

fn left_child(matrix: &Vec<Vec<i32>>, r: usize, c: usize) -> i32 {
    matrix[r+1][c]
}

fn right_child(matrix: &Vec<Vec<i32>>, r: usize, c: usize) -> i32 {
    matrix[r+1][c+1]
}

fn max_child_sum(matrix: &Vec<Vec<i32>>, r: usize, c: usize) -> i32 {
    let left = matrix[r][c] + left_child(matrix, r, c);
    let right = matrix[r][c] + right_child(matrix, r, c);
    cmp::max(left, right)
}

fn main() {
    let res = file_to_matrix("./euler/src/bin/p18-input.txt");
    if let Ok(mut tri_matrix) = res {
        for ridx in (0..tri_matrix.len() - 1).rev() {
            // compute the maximum child sum for each row of the matrix
            for cidx in 0..tri_matrix[ridx].len() {
                tri_matrix[ridx][cidx] = max_child_sum(&mut tri_matrix, ridx, cidx);
            }
        }
        println!("the maximum path sum is {}", tri_matrix[0][0]);
        print_matrix(&tri_matrix);
    } else {
        eprintln!("{}", res.unwrap_err());
    }
}