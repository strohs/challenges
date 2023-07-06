#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (
        contains_sublist(_first_list, _second_list),
        contains_sublist(_second_list, _first_list),
    ) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}

/// returns `true` if `al` is a sublist within `bl`
fn contains_sublist<T: PartialEq>(als: &[T], bls: &[T]) -> bool {
    // empty list is considered a sublist
    if als.is_empty() {
        return true;
    }

    // find all indices of als[0] in bls
    let indices: Vec<usize> = bls
        .iter()
        .enumerate()
        .filter(|el| *el.1 == als[0])
        .map(|el| el.0)
        .collect();

    for bi in indices {
        // make sure the bls slice len is <= als.len()
        let bend = bi + als.len();
        if bend > bls.len() {
            continue;
        }

        let bslice = &bls[bi..bend];
        if &als == &bslice {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{contains_sublist, sublist};
    use crate::sublist::Comparison;

    #[test]
    fn empty_list_is_sublist() {
        let al = [];
        let bl = [1, 2, 3, 4];
        assert!(contains_sublist(&al, &bl));
    }

    #[test]
    fn al_is_sublist() {
        let al = [1, 2];
        let bl = [1, 2, 3, 4];
        assert_eq!(sublist(&al, &bl), Comparison::Sublist);
    }

    #[test]
    fn al_is_not_sublist() {
        let al = [1, 3];
        let bl = [1, 2, 3, 4];
        assert!(!contains_sublist(&al, &bl));
    }

    #[test]
    fn al_is_sublist_at_end_of_bl() {
        let al = [1, 3];
        let bl = [1, 2, 5, 3, 4, 1, 3];
        assert!(contains_sublist(&al, &bl));
    }

    #[test]
    fn slices_are_equal() {
        let al = [1, 3, 4];
        let bl = [9, 1, 3, 4, 5];
        let bslice = &bl[1..4];
        assert_eq!(&al, bslice);
    }

    #[test]
    fn second_occurrence_slices_are_equal() {
        let al = [1, 3, 4];
        let bl = [9, 8, 3, 4, 5, 1, 3, 4];
        let bslice = &bl[5..8];
        assert_eq!(&al, bslice);
    }

    #[test]
    fn slices_are_not_equal() {
        let al = [1, 3, 4];
        let bl = [9, 1, 3, 8, 5];
        let bslice = &bl[1..4];
        assert_ne!(&al, bslice);
    }
}
