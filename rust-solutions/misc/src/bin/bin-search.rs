

/// basic binary search that attempts to find the element `e` within the given sorted slice
/// `sl`.
/// returns `Some(index)` containing the index of `e` within slice. If `e` was not found
/// `None` is returned
fn bin_search<T>(sl: &[T], e: T) -> Option<usize>
    where T: PartialOrd
{
    let mut bi = 0;         // begin index
    let mut ei = sl.len();  // end index, exclusive

    while let Some(cur) = sl.get(bi..ei) {
        match cur.len() {
            0 => return None,
            1 if sl[bi] == e => return Some(bi),
            1 => return None,
            _ => {
                let mp = (bi + ei) / 2;
                if e < sl[mp] {
                    ei = mp;
                } else {
                    if e == sl[mp] {
                        return Some(mp);
                    } else {
                        bi = mp;
                    }
                }
            }
        }
    }
    None
}

fn main() {

}

#[cfg(test)]
mod tests {
    use crate::bin_search;

    #[test]
    fn no_match() {
        let v = vec![0, 1];
        assert_eq!(bin_search(&v, 2), None);
    }

    #[test]
    fn no_match_at_beginning() {
        let v = vec![0, 1];
        assert_eq!(bin_search(&v, -1), None);
    }

    #[test]
    fn match_at_beginning() {
        let v = vec![0, 1];
        assert_eq!(bin_search(&v, 0), Some(0));
    }

    #[test]
    fn match_at_end() {
        let v = vec![0, 1];
        assert_eq!(bin_search(&v, 1), Some(1));
    }

    #[test]
    fn match_at_1() {
        let v = vec![0, 1, 2];
        assert_eq!(bin_search(&v, 1), Some(1));
    }

    #[test]
    fn match_at_2() {
        let v = vec![0, 1, 2];
        assert_eq!(bin_search(&v, 2), Some(2));
    }

    #[test]
    fn match_at_4() {
        let v = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(bin_search(&v, 4), Some(4));
    }

    #[test]
    fn match_at_mp() {
        let v = vec![0, 1, 2, 3, 3, 5, 6];
        assert_eq!(bin_search(&v, 3), Some(3));
    }

    #[test]
    fn match_consec() {
        let v = vec![1,1,1,1,1,1,1,1];
        assert_eq!(bin_search(&v, 1), Some(4));
    }
}