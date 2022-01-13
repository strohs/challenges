/// # 22. Generate Parentheses
/// Given `n` pairs of parentheses, write a function to generate all combinations of well-formed
/// parentheses.
///
/// Constraints: `1 <= n <= 8`
///
/// ## Example 1
/// ```
/// Input: n = 3
/// Output = ["((()))","(()())","(())()","()(())","()()()"]
/// ```
///
/// ## Example 2
/// ```
/// Input: n = 1
/// Output: ["()"]
/// ```

pub fn generate_parentheses(n: i32) -> Vec<String> {
    let mut parens: Vec<String> = vec![];
    let cur_str = String::from("");

    backtrack(&mut parens, cur_str, 0, 0, n);

    parens
}

/// helper function that recursively generates parentheses combinations. It will ensure that
/// opening parens always precede closing parens
/// ps - is a vector holding the final parentheses strings
/// cur_str = is the current parentheses string being built
/// op - a count of the current opening parentheses in cur_str
/// cp - a count of the current closing parentheses in cur_str
/// max_pairs - is the maximum number of parentheses pairs to generate
fn backtrack(ps: &mut Vec<String>, cur_str: String, op: i32, cp: i32, max_pairs: i32) {
    if cur_str.len() == (max_pairs * 2) as usize {
        // base case
        ps.push(cur_str);
        return;
    }

    if op < max_pairs {
        backtrack(ps, cur_str.clone() + "(", op + 1, cp, max_pairs);
    }

    // cant have closing parens before opening parens
    if cp < op {
        backtrack(ps, cur_str + ")", op, cp + 1, max_pairs);
    }
}

fn main() {
    let parens = generate_parentheses(3);
    dbg!(parens);
}
