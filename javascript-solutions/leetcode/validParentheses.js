/// # 20 Valid Parentheses
/// Given a string s containing just the characters `'(', ')', '{', '}', '[' and ']'`, determine
/// if the input string is valid.
///
/// An input string is valid if:
///
///     Open brackets must be closed by the same type of brackets.
///     Open brackets must be closed in the correct order.
///
/// ## Example 1
/// `Input: s = "()"`
/// `Output: true`
///
/// ## Example 2
/// `Input: s = "()[]{}"`
/// `Output: true`
///
/// ## Example 3
/// `Input: s = "(]"`
/// `Output: false`
///
/// ## Constraints
/// - 1 <= s.length <= 10^4
/// - s consists of parentheses characters only: `()[]{}`

const END_PARENS = [')',']','}'];

// map closing parens to their corresponding opening paren
const parenMap = new Map();
parenMap.set(')', '(');
parenMap.set(']', '[');
parenMap.set('}', '{');

// takes a string of parentheses and returns true if they are 'balanced', else returns false
function isValid(s) {
    const parenStack = [];

    for (const ch of s.split("")) {
        if (!END_PARENS.includes(ch)) {
            // ch is a starting paren char, so push it on the stack
            parenStack.push(ch);
        } else {
            const pch = parenStack.pop();
            if (pch === undefined || parenMap.get(ch) !== pch) {
                return false;
            }
        }
    }

    return true;
}

console.log("is valid ()", isValid("()"));  // true
console.log("is valid ()[]{}", isValid("()[]{}")); // true
console.log("is valid [}", isValid("[}"));  // false
console.log("is valid (([[{{}}]]))", isValid("(([[{{}}]]))")); // true