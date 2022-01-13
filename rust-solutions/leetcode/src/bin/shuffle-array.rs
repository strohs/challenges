extern crate rand;

use rand::Rng;

struct Solution {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution { nums }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.nums.to_vec()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&self) -> Vec<i32> {
        let mut v = self.nums.to_vec();
        let mut rng = rand::thread_rng();
        for i in (0..=v.len() - 1).rev() {
            let j: usize = rng.gen_range(0, i + 1);
            v.swap(i, j);
        }
        v
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */
fn main() {
    let nums = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let obj = Solution::new(nums);
    let ret_1: Vec<i32> = obj.reset();
    dbg!(ret_1);
    let ret_2: Vec<i32> = obj.shuffle();
    dbg!(ret_2);
}
