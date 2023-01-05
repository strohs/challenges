use std::cmp::Ordering;

/// # Problem 4 - Median of two Sorted Arrays
/// Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively,
/// return the median of the two sorted arrays.
///
/// The overall run time complexity should be O(log (m+n)).
/// ## Constraints
/// - `1 <= m + n <= 2000`
/// - `-10^6 <= nums1[i], nums2[i] <= 10^6`
///
/// I'm assuming we can't brute force this and allocated a new merged array with num1 and num2

/// given two sorted Vectors, this function walks both vectors and returns the elements
/// of the median. If both input vectors are empty, an empty Vec is returned
fn median(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    let mut medians: Vec<i32> = Vec::with_capacity(2);
    if nums1.is_empty() && nums2.is_empty() {
        return medians;
    }

    // make sure nums1 "points to" the smaller len vector
    let (nums1, nums2) = {
        if nums2.len() > nums1.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        }
    };
    let total_len = nums1.len() + nums2.len();
    // compute the midpoint
    let mp = {
        if total_len % 2 != 0 {
            // odd
            total_len / 2
        } else {
            // even
            (total_len / 2) - 1
        }
    };
    let mut i1 = 0_usize;
    let mut i2 = 0_usize;
    // count of elements visited
    let mut ev = 0_usize;

    // walk the indices to the midpoint value
    while ev < mp {
        match (nums1.get(i1), nums2.get(i2)) {
            (Some(e1), Some(e2)) => {
                match e1.cmp(e2) {
                    Ordering::Less => {
                        i1 += 1;
                        ev += 1;
                    }
                    Ordering::Greater => {
                        i2 += 1;
                        ev += 1;
                    }
                    _ => {
                        i1 += 1;
                        //i2 += 1;
                        ev += 1;
                    }
                }
            }
            (Some(_), None) => {
                i1 += 1;
                ev += 1;
            }
            (None, Some(_)) => {
                i2 += 1;
                ev += 1;
            }
            (None, None) => break,
        }
    }
    //dbg!(i1, i2);
    // now choose the actual median values from both vectors
    if total_len % 2 != 0 {
        // odd number of toal elements, median will only be one element
        match (nums1.get(i1), nums2.get(i2)) {
            (Some(e1), Some(e2)) => {
                if *e1 <= *e2 {
                    medians.push(*e1)
                } else {
                    medians.push(*e2)
                }
            }
            (Some(e1), None) => medians.push(*e1),
            (None, Some(e2)) => medians.push(*e2),
            (None, None) => (),
        }
    } else {
        // even number of total elements, median will contain 2 elements
        for _ in 0..2 {
            match (nums1.get(i1), nums2.get(i2)) {
                (Some(e1), Some(e2)) => {
                    if *e1 <= *e2 {
                        medians.push(*e1);
                        i1 += 1;
                    } else {
                        medians.push(*e2);
                        i2 += 1;
                    }
                }
                (Some(e1), None) => {
                    medians.push(*e1);
                    i1 += 1;
                }
                (None, Some(e2)) => {
                    medians.push(*e2);
                    i2 += 1;
                }
                (None, None) => (),
            }
        }
    }

    medians
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let medians = median(&nums1, &nums2);
    medians.iter().sum::<i32>() as f64 / medians.len() as f64
}

fn main() {}

#[cfg(test)]
mod tests {

    use super::find_median_sorted_arrays;
    use super::median;

    #[test]
    fn median_of_unequal_vecs() {
        let v1 = vec![1, 3, 5, 7];
        let v2 = vec![2, 4];
        let med = median(&v1, &v2);
        assert_eq!(med, vec![3, 4]);
    }

    #[test]
    fn median_when_second_vec_empty() {
        let v1 = vec![1, 3, 5, 7];
        let v2 = vec![];
        let med = median(&v1, &v2);
        assert_eq!(med, vec![3, 5]);
    }

    #[test]
    fn median_when_vec_has_one_element() {
        let v1 = vec![];
        let v2 = vec![2];
        let med = median(&v1, &v2);
        assert_eq!(med, vec![2]);
    }

    #[test]
    fn median_when_first_vec_has_one_element() {
        let v1 = vec![1];
        let v2 = vec![];
        let med = median(&v1, &v2);
        assert_eq!(med, vec![1]);
    }

    #[test]
    fn median_when_both_vecs_are_empty() {
        let v1 = vec![];
        let v2 = vec![];
        let med = median(&v1, &v2);
        assert_eq!(med, vec![]);
    }

    #[test]
    fn median_when_nums1_lt_nums2() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![5, 6, 7];
        let med = median(&v1, &v2);
        assert_eq!(med, vec![4]);
    }

    #[test]
    fn median_when_nums1_lt_nums2_part2() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6, 7];
        let med = median(&v1, &v2);
        assert_eq!(med, vec![4]);
    }

    #[test]
    fn midpoint_indices_v1_lt_v2_odd() {
        let v1 = vec![1, 2];
        let v2 = vec![3];
        let med = median(&v1, &v2);
    }

    #[test]
    fn median_when_nums1_gt_nums2() {
        let v1 = vec![90, 91, 92];
        let v2 = vec![4, 5, 6, 7];
        let med = median(&v1, &v2);
        assert_eq!(med, vec![7]);
    }

    #[test]
    fn find_median_1() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6, 7];
        let med = find_median_sorted_arrays(v1, v2);
        assert_eq!(med, 4.0);
    }

    #[test]
    fn find_median_evens() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![5, 6, 7, 8];
        let med = find_median_sorted_arrays(v1, v2);
        assert_eq!(med, 4.5);
    }

    #[test]
    fn find_median_evens_2() {
        let v1 = vec![1];
        let v2 = vec![1];
        let med = find_median_sorted_arrays(v1, v2);
        assert_eq!(med, 1.0);
    }

    #[test]
    fn find_median_of_all_zeros() {
        let v1 = vec![0, 0];
        let v2 = vec![0, 0];
        let med = find_median_sorted_arrays(v1, v2);
        assert_eq!(med, 0.0);
    }

    #[test]
    fn find_median_nums1_in_middle_of_nums2() {
        let v1 = vec![50, 51, 52, 53];
        let v2 = vec![1, 2, 3, 80, 81, 82];
        let med = find_median_sorted_arrays(v1, v2); // [51,52]
        assert_eq!(med, 51.5);
    }
}
