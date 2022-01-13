package leetcode;/// # 20 Valid Parentheses
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


import java.util.List;
import java.util.Map;
import java.util.Stack;

public class ValidParentheses {

    private final List<Character> END_PARENS = List.of(')', ']', '}');

    // using Java9 Map.of()
    private final Map<Character, Character> PAREN_MAP = Map.of(')','(', ']', '[', '}', '{');

    // returns true if s contains balanced parentheses
    public boolean isValid(String s) {
        Stack<Character> parenStack = new Stack<>();

        for (char ch : s.toCharArray()) {
            if ( !END_PARENS.contains(ch) ) {
                parenStack.push( ch );
            } else {
                if (parenStack.isEmpty()) {
                    return false;
                } else {
                    var pch = parenStack.pop();
                    if (PAREN_MAP.get(ch) != pch) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    public static void main(String[] args) {
        ValidParentheses vp = new ValidParentheses();
        System.out.println(vp.isValid("()"));       // true
        System.out.println(vp.isValid("(){}[]"));   // true
        System.out.println(vp.isValid("[}"));       // false
        System.out.println(vp.isValid("([{}])"));   // true
    }
}
