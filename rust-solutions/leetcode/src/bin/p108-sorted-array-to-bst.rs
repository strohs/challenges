use std::rc::Rc;
use std::cell::RefCell;

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


pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
  let (left, right) = nums.split_at(nums.len() / 2);
  let (mid, right) = right.split_first().unwrap();
  let mut root = None;
  insert_bst(*mid, &mut root);
  for num in left {
    insert_bst(*num, &mut root);
  }
  for num in right {
    insert_bst(*num, &mut root);
  }
  root
}

fn insert_bst(num: i32, root: &mut Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
      if num < node.borrow().val {
        let left = &mut node.borrow_mut().left;
        insert_bst(num, left);
      } else {
        let right = &mut node.borrow_mut().right;
        insert_bst(num, right);
      }

    } else {
      let _ = root.replace(Rc::new(RefCell::new(TreeNode::new(num))));
    }
}

fn main() {
  let root = sorted_array_to_bst(vec![1,3,5,7,9]);
  dbg!(root);
}