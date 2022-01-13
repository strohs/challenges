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

function generateParentheses(n) {
    const parens = [];
    let curStr = '';

    backtrack(parens, curStr, 0, 0, n);
    return parens
}


/// helper function that recursively generates parentheses combinations. It will ensure that
/// opening parens always precede closing parens
/// ps - is a array holding the final parentheses strings
/// curStr - is the current parentheses string being built
/// op - a count of the current opening parentheses in cur_str
/// cp - a count of the current closing parentheses in cur_str
/// max_pairs - is the maximum number of parentheses pairs to generate
function backtrack(ps, curStr, op, cp, max_pairs) {
    if (curStr.length === max_pairs * 2) {
        ps.push(curStr);
        return;
    }

    if (op < max_pairs) {
        backtrack(ps, curStr + '(', op + 1, cp, max_pairs);
    }

    if (cp < op) {
        backtrack(ps, curStr + ')', op, cp + 1, max_pairs);
    }
}

const res = generateParentheses(3);
res.forEach(s => console.log(s));