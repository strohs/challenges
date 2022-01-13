///
/// radix sort for sorting a list of positive integers
///
///
///


// use rand::Rng;
use std::collections::{VecDeque};

fn length(i: i64) -> usize {
    i.abs().to_string().len()
}

fn digit_at( i:i64, pos:usize ) -> usize {
    if pos > length(i) {
        return 0;
    }
    let mut num = i;
    let mut rem = 0;
    for _j in 0..pos {
        rem = num % 10;
        num /= 10;
    }
    rem as usize
}

//fn gen_random_ints( length:u32 ) -> Vec<u32> {
//    let mut rng = rand::thread_rng();
//    let mut nums: Vec<u32> = Vec::new();
//    for _i in 0..length {
//        nums.push( rng.gen() );
//    }
//    return nums;
//}

fn build_digit_buckets() -> Vec<VecDeque<i64>> {
    let buckets: Vec<VecDeque<i64>> = vec![VecDeque::new(); 10];
    buckets
}

pub fn sort(list: &mut Vec<i64>) -> &mut Vec<i64> {

    let mut buckets: Vec<VecDeque<i64>> = build_digit_buckets();

    // 1. find max number of digits among all integers in the vector to be sorted
    let max_length = list.iter()
        .map(|n| length(*n) )
        .max_by(|a,b| a.cmp(b))
        .unwrap();

    println!("largest digit length {}", max_length);

    // 2. outer loop will loop through digits of each number, starting with least significant digits
    for pos in 1..=max_length {
        for num in list.iter() {
            // 3. store the number in the bucket based on its least significant digit
            let digit = digit_at( *num, pos);
            buckets[digit].push_back( *num );
        }
        list.clear();
        // 4. remove numbers from buckets, in order 0-9 and store back in the list
        for vec_deq in &mut buckets {
            while let Some(num) = vec_deq.pop_front() {
                list.push( num );
            }
        }
    }
    list
}

fn main() {}

#[cfg(test)]
mod tests {

    use super::sort;

    #[test]
    fn radix_sort_ints() {
        let mut ints: Vec<i64> = vec![1354, 56353, 6345, 13455, 33553146, 2, 1, 99, 55];
        sort(&mut ints);
        assert_eq!( *ints.first().unwrap(), 1);
        assert_eq!( *ints.last().unwrap(), 33553146);
    }
}
