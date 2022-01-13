/// rotate an the values in a square array by 90 "degrees" clockwise
/// The input matrix will be stored as a 1-D array with a dimension, dim
///
/// # Example
/// Given:
/// ```
/// 1 2 3
/// 4 5 6
/// 7 8 9
/// ```
///
/// 1. transpose the matrix along the diagonal running from 0,0 to n-1,n-1
/// transpose takes rows and turns them into columns:
/// 1 4 7
/// 2 5 8
/// 3 6 9
///
/// 2. then flip the transposed matrix, horizontally (imagine a vertical line thru column n/2
/// 7 4 1
/// 8 5 2
/// 9 6 3
///

// transpose a square matrix stored as a 1D array
fn transpose(arr: &mut [i32], dim: usize) {
    // convert 2d coordinates (row major) to 1d
    let to1d = |row: usize, col: usize| row * dim + col;

    for r in 0..dim {
        for c in 0..dim {
            let i = to1d(r, c);
            // only swap indices above the array's diagonal
            if c > r {
                //covert 2d indices to 1d
                arr.swap(i, c * dim + r);
            }
        }
    }
}

/// flip a matrix stored as a 1D array
fn horizontal_flip(arr: &mut [i32], dim: usize) {
    // convert 2d coordinates (row major) to 1d
    let to1d = |row: usize, col: usize| row * dim + col;

    for r in 0..dim {
        for c in 0..dim {
            let i = to1d(r, c);

            // only examine half the columns
            if c < dim / 2 {
                let si = i + (dim - c - 1);
                arr.swap(i, si);
            }
        }
    }
}

fn print(arr: &[i32], dim: usize) {
    for (i, _item) in arr.iter().enumerate() {
        print!("{} ", arr[i]);
        if (i + 1) % dim == 0 {
            println!();
        }
    }
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    transpose(&mut arr, 3);
    horizontal_flip(&mut arr, 3);

    print(&arr, 3);
}
