/// # Problem 56 - Merge Intervals
/// Given an array of `intervals` where `intervals[i] = [starti, endi]`, merge all
/// overlapping intervals, and return an array of the non-overlapping intervals that
/// cover all the intervals in the input.
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    // sort by the first element of each sub-vector
    intervals.sort_unstable_by_key(|int| int[0]);

    let mut res = vec![];
    let mut i = 0_usize;
    while i < intervals.len() {
        let j = i + 1;
        if j < intervals.len() {
            if intervals[i][1] >= intervals[j][0] {
                intervals[j][0] = intervals[i][0];
                if intervals[i][1] >= intervals[j][1] {
                    intervals[j][1] = intervals[i][1];
                }
            } else {
                res.push(intervals[i].clone());
            }
        } else {
            res.push(intervals[i].clone());
        }
        i += 1;
    }

    res
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::merge;

    #[test]
    fn example1() {
        let ints = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let merged = merge(ints);
        assert_eq!(merged, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn example2() {
        let ints = vec![vec![1, 3], vec![2, 6]];
        let merged = merge(ints);
        assert_eq!(merged, vec![vec![1, 6]]);
    }

    #[test]
    fn example3() {
        let ints = vec![vec![1, 3]];
        let merged = merge(ints);
        assert_eq!(merged, vec![vec![1, 3]]);
    }

    #[test]
    fn example4() {
        let ints = vec![vec![1, 3], vec![2, 6], vec![8, 10]];
        let merged = merge(ints);
        assert_eq!(merged, vec![vec![1, 6], vec![8, 10]]);
    }

    #[test]
    fn example5() {
        let ints = vec![vec![1, 4], vec![0, 4]];
        let merged = merge(ints);
        assert_eq!(merged, vec![vec![0, 4]]);
    }

    #[test]
    fn example6() {
        let ints = vec![vec![1, 4], vec![2, 3]];
        let merged = merge(ints);
        assert_eq!(merged, vec![vec![1, 4]]);
    }
}
