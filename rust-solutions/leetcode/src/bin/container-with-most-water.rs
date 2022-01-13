/// # 11. Container with the most water
/// Given `n` non-negative integers `a1, a2, ..., an` , where each represents a point at
/// coordinate (i, ai). `n` vertical lines are drawn such that the two endpoints of line i is
/// at (i, ai) and (i, 0). Find two lines, which together with x-axis forms a container, such
/// that the container contains the most water.
/// **NOTE**: You may not slant the container and n is at least 2.
///
/// ## Example
/// `Input: [1, 8, 6, 2, 5, 4, 8, 3, 7]`
/// `Output: 49`
///

/// brute force solution to find max area: O(n^2)
fn max_area(arr: &[i32]) -> usize {
    fn x_dist(start_idx: usize, cur_idx: usize) -> usize {
        cur_idx - start_idx
    }
    let mut max_area = 0_usize;

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            let cur_area = {
                if arr[i] < arr[j] {
                    arr[i] as usize * x_dist(i, j)
                } else {
                    arr[j] as usize * x_dist(i, j)
                }
            };
            if cur_area > max_area {
                println!(
                    "old: {}  new: {} at index:({},{})",
                    max_area, cur_area, i, j
                );
                max_area = cur_area;
            }
        }
    }
    max_area
}

fn main() {
    let arr = [1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("max area {}", max_area(&arr)); // 49

    let arr2 = [1, 6, 2, 5, 8, 7, 4, 8, 3];
    println!("max area {}", max_area(&arr2)); // 49
}
