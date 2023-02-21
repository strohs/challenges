use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;

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

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  helper(&root)
}

fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
  if let Some(node) = root {
    let n: &RefCell<TreeNode> = node.borrow();
    let l = &n.borrow().left;
    let r = &n.borrow().right;
    1 + std::cmp::max(helper(l), helper(r))
  } else {
    0
  }
}

fn main() {}