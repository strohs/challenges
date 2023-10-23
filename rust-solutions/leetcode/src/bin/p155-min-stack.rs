

/// # Leetcode - Problem 155 - Min Stack
/// https://leetcode.com/problems/min-stack/
/// This solution uses a Vec as the stack with elements stored in reverse order.
/// The minimum element will be at the top of the stack to enable pop in O(1) time.
/// rust's binary_search function is used to find insertion points and keep elements in (reverse)
/// sorted order
/// - Methods pop, top and getMin operations will always be called on non-empty stacks.
struct MinStack {
    stack:Vec<(i32, i32)>,
}

impl MinStack {

    fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let nn =  match &self.stack.last() {
            Some((_tail_val, tail_min)) => (val, std::cmp::min(val, *tail_min)),
            None => (val, val),
        };
        self.stack.push(nn);
    }

    fn pop(&mut self) {
        self.stack.pop().expect("stack will have at least one element");
    }

    // returns a copy of the tail value in the stack, does not pop it
    fn top(&self) -> i32 {
        self.stack.last().expect("stack will have at least one element").0
    }

    // the current minimum value in the stack
    fn get_min(&self) -> i32 {
        self.stack.last().expect("stack will have at least one element").1
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
fn main() {}

#[cfg(test)]
mod tests {
    use crate::MinStack;

    #[test]
    fn test_example1() {
       let mut ms = MinStack::new();
        ms.push(-2);
        ms.push(0);
        ms.push(-3);
        assert_eq!(ms.stack.len(), 3);
        assert_eq!(ms.get_min(), -3);
        ms.pop();
        assert_eq!(ms.top(), 0);
        assert_eq!(ms.get_min(), -2);
    }

}