/// Generate (print) the permutations of a string

fn main() {
    permute("abc");
}

fn permute(s: &str) {
    perm_rec(String::new(), s.to_string())
}

fn perm_rec(perm: String, rest: String) {
    match rest.len() {
        0 => println!("{}", &perm),
        _ => {
            for i in 0..rest.len() {
                let c = rest.chars().nth(i).unwrap();
                // build the rest string without the char at position i
                let new_rest = format!("{}{}", &rest[0..i], &rest[i+1..]);

                perm_rec(perm.clone() + &c.to_string(), new_rest);
            }
        }
    }
}