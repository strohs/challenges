use std::cell::RefCell;
use std::rc::Rc;

/// # Path Sum II
/// [Problem Description](https://leetcode.com/problems/path-sum-ii/)
///


// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}


pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {

    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, target_sum: i32, mut cur_path: Vec<i32>, res: &mut Vec<Vec<i32>>) {
        match root {
            None => (),
            Some(node) => {
                let cur_val = *&node.borrow().val;
                cur_path.push(cur_val);
                match (&node.borrow().left, &node.borrow().right) {
                    (None, None) if target_sum - cur_val == 0 => {
                        res.push(cur_path.clone());
                    },
                    (None, None) => (),
                    (Some(left), None) => {
                        let new_target = target_sum - *&node.borrow().val;
                        helper(Some(left), new_target, cur_path, res);
                    },
                    (None, Some(right)) => {
                        let new_target = target_sum - *&node.borrow().val;
                        helper(Some(right), new_target, cur_path, res);
                    },
                    (Some(left), Some(right)) => {
                        let new_target = target_sum - *&node.borrow().val;
                        helper(Some(left), new_target, cur_path.clone(), res);
                        helper(Some(right), new_target, cur_path.clone(), res);
                    }
                }
            }
        }
    }

    let mut res: Vec<Vec<i32>> = vec![];
    helper(root.as_ref(), target_sum, vec![], &mut res);
    res
}


fn main() {}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::{path_sum, TreeNode};

    #[test]
    fn test1() {
        let mut node = TreeNode::new(5);
        node.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = Some(Rc::new(RefCell::new(node)));

        let res = path_sum(root, 8);
        dbg!(res);
    }

}