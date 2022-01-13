
/// Given a vector of sorted integers, returns a sorted vector of the squares of those integers
fn sort_and_square_array(ints: &[i32]) -> Vec<i32> {
    if ints.is_empty() {
        return vec![];
    }
    // sqs will hold the final sorted squared array
    let mut sqs: Vec<i32> = vec![0; ints.len()];

    let mut fi: usize = 0;              // pointer to front index
    let mut ti= ints.len() - 1;  // pointer to tail index

    // compare the front and tail indices and insert the larger of the their values into the end
    // of the final array
    for si in (0..(ints.len())).rev()  {
        let fv = ints[fi] * ints[fi];
        let tv = ints[ti] * ints[ti];
        if fv >= tv {
            sqs[si] = fv;
            fi += 1;
        } else {
            sqs[si] = tv;
            ti -= 1;
        }
    }
    sqs
}

fn main() {
    let v1 = vec![-10000, -5, -3, 1, 2, 4, 9999];
    let v2 = vec![-5];
    dbg!("v1: {}", sort_and_square_array(&v1));
    dbg!("v2: {}", sort_and_square_array(&v2));
}
