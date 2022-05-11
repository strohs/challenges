/// # Problem 57 - Insert Interval
/// You are given an array of non-overlapping intervals `intervals` where `intervals[i] = [starti, endi]`
/// represent the start and the end of the ith interval and `intervals` is sorted in ascending order by
/// `start_i`. You are also given an interval `newInterval = [start, end]` that represents the start and
/// end of another interval.
///
/// Insert `newInterval` into `intervals` such that `intervals` is still sorted in ascending order by
/// `start_i` and `intervals` still does not have any overlapping intervals (merge overlapping intervals
/// if necessary).
///
/// Return `intervals` after the insertion.
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    let mut cidx = 0_usize;
    let (ns, ne) = (new_interval[0], new_interval[1]);

    while cidx < intervals.len() {
        // current_interval_start, cs,  and current_interval_end, ce
        let (cs, ce) = (intervals[cidx][0], intervals[cidx][1]);

        if ns < cs && ne < cs {
            // new interval is completely in front of current interval
            intervals.insert(cidx, new_interval);
            return intervals;
        }
        // check if new interval completely within an existing interval
        if ns >= cs && ne <= ce {
            return intervals;
        }

        // check for an overlap
        if (ns >= cs && ns <= ce) || (ne <= ce && ne >= cs) || (ns < cs && ne > ce) {
            let start = ns.min(cs);
            // new_end is < current_end
            if ne <= ce {
                intervals[cidx][0] = start;
                intervals[cidx][1] = ce;
                return intervals;
            }

            // need to find insertion index and possibly do a interval merge
            let mut ins_idx = intervals[cidx..]
                .iter()
                .enumerate()
                .find(|(_i, inter)| inter[0] > ne || (ne >= inter[0] && ne <= inter[1]))
                .map_or_else(|| intervals.len(), |(i, _inter)| i + cidx); // + cidx becuase we started the find from cidx

            let end = if ins_idx == intervals.len() {
                ins_idx -= 1;
                ne
            } else if ne >= intervals[ins_idx][0] && ne <= intervals[ins_idx][1] {
                ne.max(intervals[ins_idx][1])
            } else {
                ins_idx -= 1;
                ne.max(intervals[ins_idx][1])
            };
            let replacement = vec![start, end];
            intervals.splice(cidx..=ins_idx, [replacement]);
            return intervals;
        }
        cidx += 1;
    }
    // if we end up here, then new_interval falls completely after the last interval in intervals,
    // so just insert new_iterval at the end
    intervals.insert(cidx, new_interval);
    intervals
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::insert;

    #[test]
    fn example1() {
        let intervals = vec![vec![1,3], vec![6,9]];
        let new_int = vec![2, 5];
        assert_eq!(insert(intervals, new_int), vec![vec![1,5], vec![6,9]]);
    }

    #[test]
    fn example2() {
        let intervals = vec![vec![1,2], vec![3,5], vec![6,7], vec![8, 10], vec![12,16]];
        let new_int = vec![4,8];
        assert_eq!(insert(intervals, new_int), vec![vec![1,2], vec![3, 10], vec![12, 16]]);
    }

    #[test]
    fn example3() {
        let intervals = vec![vec![1,2], vec![3,5], vec![6,7], vec![8, 10], vec![12,16]];
        // within last interval
        let new_int = vec![13,14];
        assert_eq!(insert(intervals, new_int), vec![vec![1,2], vec![3,5], vec![6,7], vec![8, 10], vec![12,16]]);
    }

    #[test]
    fn example4() {
        let intervals = vec![vec![1,3], vec![6,9]];
        let new_int = vec![12, 16];
        assert_eq!(insert(intervals, new_int), vec![vec![1,3], vec![6,9], vec![12, 16]]);
    }

    #[test]
    fn example5() {
        let intervals = vec![vec![1,3], vec![6,9]];
        let new_int = vec![4, 5];
        assert_eq!(insert(intervals, new_int), vec![vec![1,3], vec![4,5], vec![6,9]]);
    }

    #[test]
    fn example6() {
        let intervals = vec![vec![5,9], vec![13,15]];
        let new_int = vec![1,3];
        assert_eq!(insert(intervals, new_int), vec![vec![1,3], vec![5,9], vec![13, 15]]);
    }

    #[test]
    fn example7_empty_input() {
        let intervals = vec![];
        let new_int = vec![1,3];
        assert_eq!(insert(intervals, new_int), vec![vec![1,3]]);
    }

    #[test]
    fn example8() {
        let intervals = vec![vec![1,5]];
        let new_int = vec![0,3];
        assert_eq!(insert(intervals, new_int), vec![vec![0,5]]);
    }

    #[test]
    fn example9() {
        let intervals = vec![vec![1,5]];
        let new_int = vec![0,6];
        assert_eq!(insert(intervals, new_int), vec![vec![0,6]]);
    }

    #[test]
    fn example10() {
        let intervals = vec![vec![1,5], vec![6,8]];
        let new_int = vec![3,7];
        assert_eq!(insert(intervals, new_int), vec![vec![1,8]]);
    }

    #[test]
    fn example11() {
        let intervals = vec![vec![1,5], vec![6,8]];
        let new_int = vec![0,9];
        assert_eq!(insert(intervals, new_int), vec![vec![0,9]]);
    }

    #[test]
    fn example12() {
        let intervals = vec![vec![1,5], vec![9,12]];
        let new_int = vec![0,4];
        assert_eq!(insert(intervals, new_int), vec![vec![0,5], vec![9,12]]);
    }

    #[test]
    fn example13() {
        let intervals = vec![vec![0,0], vec![2,4], vec![9,9]];
        let new_int = vec![0,7];
        assert_eq!(insert(intervals, new_int), vec![vec![0,7], vec![9,9]]);
    }
}