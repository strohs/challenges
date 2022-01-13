
/// Dan is playing a video game in which his character competes in a hurdle race. Hurdles are of
/// varying heights, and Dan has a maximum height he can jump. There is a magic potion he can take
/// that will increase his maximum height by 1 unit for each dose. How many doses of the potion
/// must he take to be able to jump all of the hurdles.
///
/// Given an array of hurdle heights, height, and an initial maximum height Dan can jump, k,
/// determine the minimum number of doses Dan must take to be able to clear all the hurdles in the
/// race.
///
/// # Example
/// if height=[1,2,3,3,2] and Dan can jump 1 unit high naturally, he must take 3 - 1 = 2 doses
/// of potion to be able to jump all of the hurdles.
///


/// return the minimum units of potion Dan needs to drink to jump all of the hurdles.
///
/// # Params
/// * k: an integer denoting the height Dan can jump naturally
/// * height: an array of integers denoting the heights of each hurdle
///
/// # Examples
/// ```
/// assert_eq!(hurdle_race(4, &[1,6,3,5,2]), 2)
/// assert_eq!(hurdle_race(7, &[2,6,4,5,2]), 0)
/// ```
fn hurdle_race(k: u32, height: &[u32]) -> u32 {
    let mut max_jump = k;
    let mut dose_count = 0;
    for &hurdle_height in height {
        if hurdle_height > max_jump {
            dose_count += hurdle_height - max_jump;
            max_jump += dose_count;
        }
    }
    dose_count
}

fn main() {
    dbg!( hurdle_race(4, &[1,6,3,5,2]) );
    dbg!( hurdle_race(7, &[2,6,4,5,2]) );
}