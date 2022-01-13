/// write a function that prints all subsets of a set to standard out
/// the set contains unique integers
/// # Example
/// Given:
///     `[1]`  the subsets are  `[], [1]`
/// Given:
///     `[1,2]` the subsets are  `[], [1,2], [1], [2]`
/// Given:
///     `[1,2,3]`  the subsets are  `[], [1,2,3], [1,2], [2,3], [1,3], [1], [2], [3]`
///
///     1,2,3,4    1,2,3,4   1,2,3  2,3,4  1,3,4  1,2,4   1,2  2,3  3,4  1,4  1,3  2,4  1  2  3  4
///
/// in general it seems the number of subsets = 2^n  where n is the number of elements in the set


/// iterative powerset using a bitshift index
// fn powerset<T>(s: &[T]) -> Vec<Vec<T>> where T: Clone {
//     (0..2usize.pow(s.len() as u32)).map(|i| {
//         s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
//             .map(|(_, element)| element.clone())
//             .collect()
//     }).collect()
// }
//
// fn main() {
//     let v = vec![1,2,3,4];
//     println!("{:?}", v);
//     let pset = powerset(&v);
//     println!("{:?}", pset);
// }



fn main() {
    let s1 = vec![1,2,3];
    power_set(&s1);
}

fn power_set(set: &Vec<i32>) {
    fn recurse( set: &Vec<i32>, pos: usize, mut prefix: Vec<i32>) {
        //println!("pos:{} prefix:{:?}", &pos, &prefix);
        prefix.push(set[pos]);
        println!("{:?}",&prefix);
        for i in pos+1..set.len() {
            let newprefix = prefix.clone();
            recurse(set,i, newprefix);
        }
    }

    for i in 0..set.len() {
        let prefix: Vec<i32> = Vec::new();
        recurse(  &set,i, prefix );

    }
    println!("[]");
}

