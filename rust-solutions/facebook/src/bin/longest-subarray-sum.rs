/// given an array of integers, of some length, N, find the longest sub-array of integers that sum to some
/// value, S.  If there is no such sub-array, return an empty array
/// Ex.  arr = `[1,3,5,4,7,5] S=12`
///     the longest subarray is `[3,5,4]`

fn find_longest_subarr_sum(nums: &[i32], s: i32) -> Vec<i32> {
    let mut longest: Vec<i32> = vec![];

    for csi in 0..nums.len() {
        let mut i = csi;
        let mut diff = s;

        while i < nums.len() && diff >= 0 {
            diff -= nums[i];
            i += 1;
            if diff == 0 && i - csi > longest.len() {
                longest = nums[csi..i].to_owned()
            }
        }
    }
    longest
}

fn main() {
    println!("{:?}", find_longest_subarr_sum(&[1,3,5,4,8,5,7], 12));
}

#[cfg(test)]
mod test {
    use crate::find_longest_subarr_sum;

    #[test]
    fn non_zero_vec() {
        let nums = vec![1,3,5,4,8,5,7];
        let s = 12;
        let longest = find_longest_subarr_sum(&nums, s);
        assert_eq!(longest, vec![3,5,4]);
    }

    #[test]
    fn vec_with_zeroes() {
        let nums = vec![1,3,5,4,0,0,8,5,7];
        let s = 12;
        let longest = find_longest_subarr_sum(&nums, s);
        assert_eq!(longest, vec![3,5,4,0,0]);
    }

    #[test]
    fn vec_does_not_sum_to_12() {
        let nums = vec![1,3,2,3,0,0,8,2];
        let s = 12;
        let longest = find_longest_subarr_sum(&nums, s);
        assert_eq!(longest, vec![]);
    }

    #[test]
    fn vec_has_number_12() {
        let nums = vec![1,3,2,3,1,1,12,2];
        let s = 12;
        let longest = find_longest_subarr_sum(&nums, s);
        assert_eq!(longest, vec![12]);
    }
}