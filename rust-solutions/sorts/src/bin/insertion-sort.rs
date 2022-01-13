use std::cmp::Ordering;

/// basic insertion sort implementation that sorts elements of a slice into ASCENDING order


// inserts the element at `idx` in the given `arr`, into its corrected sorted position
// The elements of `arr` from index 0 to `idx` (exclusive) must already be in sorted order.
fn insert<T: PartialOrd>(idx: usize, arr: &mut[T]) {
    for i in (0..=idx).rev() {
        if i == 0 {
            break;
        }
        match arr[i].partial_cmp(&arr[i-1]) {

            Some(Ordering::Less) => arr.swap(i, i-1),

            // array is already in sorted order, so we are done
            _ => break,
        }
    }
}

// Sorts the elements of arr using an insertion sort
fn sort<T: PartialOrd>(arr: &mut[T]) {
    for i in 1..arr.len() {
        insert(i, arr);
    }
}


fn main() {
    let mut arr = [15,3,12,6,9,9];
    sort(&mut arr);
    println!("{:?}", &arr);
}

#[cfg(test)]
mod tests {
    use crate::insert;

    #[test]
    fn test_insert() {
        let mut arr = [3,6,9,1];
        insert(3, &mut arr);
        assert_eq!(arr, [1,3,6,9]);
    }

    #[test]
    fn test_insert_array_length_1() {
        let mut arr = [3];
        insert(0, &mut arr);
        assert_eq!(arr, [3]);
    }

    #[test]
    fn test_insert_array_already_sorted() {
        let mut arr = [3,6,9,12];
        insert(0, &mut arr);
        assert_eq!(arr, [3,6,9,12]);
    }
}