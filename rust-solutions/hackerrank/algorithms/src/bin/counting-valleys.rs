
fn valley_count(s: &str) -> u32 {
    let mut cur_height = 0;
    let mut valley_count = 0;
    for c in s.chars() {
        match c {
            'U' => cur_height += 1,
            'D' => cur_height -= 1,
            _ => panic!("unknown char in input string"),
        }
        if cur_height == 0 && c == 'U' {
            valley_count += 1;
        }
    }
    return valley_count
}

fn main() {
    let s = "UUDDDDUU";
    let s2 = "DUDUDU";
    println!("{}", valley_count(s));
    println!("{}", valley_count(s2));
}