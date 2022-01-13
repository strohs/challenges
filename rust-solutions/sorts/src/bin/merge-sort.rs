
/// combine two (sorted) slices into a new vector, copying the values
fn combine<T>(l: &[T], r: &[T]) -> Vec<T>
    where T: PartialOrd + Copy {

    let mut combs: Vec<T> = Vec::new();
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < l.len() && j < r.len() {
        if l[i].le(&r[j]) {
            combs.push(l[i]);
            i += 1;
        } else {
            combs.push(r[j]);
            j += 1;
        }
    }

    while i < l.len() {
        combs.push(l[i]);
        i += 1;
    }
    while j < r.len() {
        combs.push(r[j]);
        j += 1;
    }

    combs
}

/// sort arr using merge sort
fn merge_sort<T>(arr: &[T]) -> Vec<T>
    where T: PartialOrd + Copy {

    if arr.len() <= 1 {
        return arr.to_vec();
    }
    // compute the midpoint
    let mp = arr.len() / 2;
    // recursively sort left and right slices
    let vl = merge_sort(&arr[..mp]);
    let vr = merge_sort(&arr[mp..]);

    // finally combine the left and right halves
    combine(&vl, &vr)
}

fn main() {
    let mut arr = [9,7,5,2,0,8,6,4,3,1];

    let (a1, a2) = arr.split_at_mut(5);
    dbg!(&a1);
    dbg!(&a2);

    //dbg!( merge_sort(&arr.to_vec()) );
}