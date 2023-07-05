/// # Leetcode 150 - Evaluate Reverse Polish Notation
/// https://leetcode.com/problems/evaluate-reverse-polish-notation/

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        let pop_two = |stack: &mut Vec<i32>| {
            let n1 = stack.pop().expect("stack has one number");
            let n2 = stack.pop().expect("stack has two numbers");
            (n1, n2)
        };

        for tok in &tokens {
            match tok.as_str() {
                "+" => {
                    let (n1, n2) = pop_two(&mut stack);
                    stack.push(n2 + n1);
                },
                "-" => {
                    let (n1, n2) = pop_two(&mut stack);
                    stack.push(n2 - n1);
                },
                "*" => {
                    let (n1, n2) = pop_two(&mut stack);
                    stack.push(n2 * n1);
                },
                "/" => {
                    let (n1, n2) = pop_two(&mut stack);
                    stack.push(n2 / n1);
                },
                _ => {
                    let num: i32 = tok.parse().expect("valid i32");
                    stack.push(num);
                },
            }
        }

        stack.pop().expect("stack will have one final result element")
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Solution;

    // macro_rules! vec_string {
    //     () => {
    //
    //     };
    // }

    #[test]
    fn test_addition() {
        let toks = vec![String::from("5"), String::from("3"), String::from("+")];
        assert_eq!(Solution::eval_rpn(toks), 8);
    }

    #[test]
    fn test_ex1() {
        let toks = vec![String::from("2"), String::from("1"), String::from("+"), String::from("3"), String::from("*")];
        assert_eq!(Solution::eval_rpn(toks), 9);
    }

    #[test]
    fn test_one_element() {
        let toks = vec![String::from("5")];
        assert_eq!(Solution::eval_rpn(toks), 5);
    }
}