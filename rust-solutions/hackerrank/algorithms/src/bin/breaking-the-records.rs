
/*
Maria plays college basketball and wants to go pro. Each season she maintains a record of her play. She tabulates
the number of times she breaks her season record for most points and least points in a game. Points scored in the
first game establish her record for the season, and she begins counting from there.

For example, assume her scores for the season are represented in the array scores = [12,24,10,24]

Given Maria's scores for a season, find and print the number of times she breaks her records for most and least
points scored during the season.
 */

/**
 * Complete the breakingRecords function in the editor below. It must return an integer array containing the numbers
 * of times she broke her records. Index 0 is for breaking most points records, and index 1 is for breaking least
 * points records.
 */
fn breaking_records(arr: &[u32]) -> [u32;2] {
    let mut cur_max = arr[0];
    let mut cur_min = arr[0];
    let mut mpc:u32 = 0;
    let mut lpc:u32 = 0;

    for x in arr.iter() {
        if x > &cur_max {
            mpc += 1;
            cur_max = *x;
        } else if x < &cur_min {
            lpc += 1;
            cur_min = *x;
        }
    }
    return [mpc,lpc]
}

fn main() {
    let arr = [10, 5, 20, 20, 4, 5, 2, 25, 1];
    let counts = breaking_records(&arr);
    println!("{} : {}", counts[0], counts[1])
}