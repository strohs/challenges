
/// # Problem 24 - Lexicographic Permutations
///
/// A permutation is an ordered arrangement of objects. For example, 3124 is one possible
/// permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or
/// alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
///
/// 012 021 102 120 201 210
/// 012 021 102
/// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?


fn main() {
    let mut a = [0, 1, 2,3,4,5,6,7,8,9];

    let res = lex_perm(1_000_000, &mut a);
    println!("result is {}", res); //2783915460
}

// find the index of the smallest element in `a` that is to the right of first and is also > first
fn smallest_pos(a: &[u8], first: u8, b: usize) -> usize {

    let smallest = a[b+1..].iter()
        .enumerate()
        .scan(b,|ci, (idx, e)| {
            if *e > first && *e < a[*ci] {
                *ci = idx + b + 1;
            }
            Some(*ci)
        });

    smallest.last().unwrap_or(b)
    // let mut ci = b;
    // for i in b+1..a.len() {
    //     if a[i] > first && a[i] < a[ci] {
    //         ci = i;
    //     }
    // }
    // ci
}

fn lex_perm(nth: usize, a: &mut[u8]) -> String {
    let size = a.len();
    let mut n = nth;

    while n > 1 {
        println!("{:?}", &a);

        // find rightmost char which is smaller than its next char, call it 'firstChar'
        if let Some(i) = a.windows(2).rposition(|sl| sl[0] < sl[1]) {

            // find ceiling of first char to right of first char
            let ceil_idx = smallest_pos(&a,a[i], i+1);
            
            // swap first and second chars
            a.swap(i, ceil_idx);

            // sort element to the right of 'firstChar'
            a[(i + 1)..size].sort();

            n = n - 1;
        } else {
            n = 0;
        }
    }

    a.iter().map(|d| d.to_string() ).collect()
}

// Heap's algorithm to generate permutations
// fn heap_perm(nth: usize, a: &mut[u8]) {
//     //c is an encoding of the stack state.
//     // c[k] encodes the for-loop counter for when generate(k+1, A) is called
//     let mut c: Vec<usize> = vec![0; a.len()];
//
//     println!("{:?}", &a);
//
//     let mut i = 0;
//     while i < nth {
//         if c[i] < i {
//             if i % 2 == 0 {
//                 a.swap(0, i);
//             } else {
//                 a.swap(c[i], i);
//             }
//             println!("{:?}", &a);
//             c[i] = c[i] + 1;
//             i = 0;
//         } else {
//             c[i] = 0;
//             i = i + 1;
//         }
//     }
// }
//