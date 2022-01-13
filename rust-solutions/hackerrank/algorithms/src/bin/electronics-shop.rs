use std::fs::File;
use std::io::{BufRead, BufReader, Result};

///
/// brute force solution O(m * n)
///
fn gms_bf(kbs: &Vec<u32>, ds: &Vec<u32>, budget: u32) -> u32 {
    let mut closest_pair = [0,0];
    let mut cur_closest = std::u32::MAX;

    for keyboard in kbs.iter() {
        for drive in ds.iter() {
            if (keyboard + drive <= budget) && (budget - (keyboard + drive) < cur_closest) {
                cur_closest = budget - (keyboard + drive);
                closest_pair[0] = *keyboard;
                closest_pair[1] = *drive;
                println!("closest pair:{:?}", &closest_pair);
            }
        }
    }
    println!("return closest pair:{:?}", &closest_pair);
    closest_pair.iter().sum()
}

fn get_money_spent( keyboards: &mut Vec<i32>, drives: &mut Vec<i32>, budget: i32) -> i32 {
    keyboards.sort_unstable_by(|a,b| a.cmp(b).reverse() );
    drives.sort_unstable();

    let mut closest_pair = [-1,-1];
    let mut cur_closest = 999999999;
    let ls: &mut Vec<i32>; // larger of the two lists
    let ss: &mut Vec<i32>; // smaller of the two lists

    if keyboards.len() > drives.len() {
        ls = keyboards;
        ss = drives;
    } else {
        ss = keyboards;
        ls = drives;
    }
    for keyboard in ls.iter() {
        let drive = ss
            .iter()
            .find(|&d| (keyboard + d <= budget) && (budget - (keyboard + d) < cur_closest) );
        match drive {
            Some(price) => {
                cur_closest = budget - (keyboard + price);
                closest_pair[0] = *keyboard;
                closest_pair[1] = *price;
                println!("closest pair:{:?}",&closest_pair);
            },
            _ => (),
        }
    }
    println!("return closest pair:{:?}",&closest_pair);
    return closest_pair.iter().sum();
}


fn main() {
    //let res1 = get_money_spent( &mut vec![3,1], &mut vec![5,2,8], 10);
    let res1 = gms_bf( &vec![3,1], &vec![5,2,8], 10);
    println!("res:{}",res1);
//    let path = "/home/cliff/idea-projects/hackerrank/rust-challenges/algorithms/src/bin/elect-shop.txt";
//    let reader = BufReader::new(File::open(path).expect("Cannot open file.txt"));
//
//    let mut keyboards: Vec<i32> = Vec::new();
//    let mut drives: Vec<i32> = Vec::new();
//
//
//    // do two times
//    let mut lineCount = 0;
//    for line in reader.lines() {
//        for word in line.unwrap().split_whitespace() {
//            if lineCount == 0 {
//                keyboards.push( word.parse().unwrap() );
//            }
//            if lineCount == 1 {
//                drives.push( word.parse().unwrap() );
//            }
//        }
//        lineCount += 1;
//    }
}