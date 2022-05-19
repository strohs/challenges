/// # Problem 61 - Rotate List
/// Given the head of a linked list, rotate the list to the right by `k` places.
///
/// The number of nodes in the list is in the range `[0, 500]`.
/// `-100 <= Node.val <= 100`
/// `0 <= k <= 2 * 109`

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {

    fn len(mut head: &Option<Box<ListNode>>) -> usize {
        let mut len = 0;
        while let Some(node) = head {
            len += 1;
            head = &node.next;
        };
        len
    }

    // find the length of the entire list
    let length = len(&head);
    if length == 0 || length == 1 {
        return head;
    }

    let mut head = head.unwrap();

    // determine the index that will be the new head after rotating
    let nh_idx = length - k as usize % length;
    if nh_idx == 0 || nh_idx == length {
        // no rotation needed as current head will rotate right back to its current position
        return Some(head);
    }

    // walk cur to the node previous to the nh_idx node
    let mut cur = &mut head;
    for _ in 0..nh_idx-1 {
        cur = cur.next.as_mut().unwrap();
    }

    // swap nodes, res will link to cur.next, which will become the new head of the rotated list
    let mut res = std::mem::replace(&mut cur.next, None).unwrap();
    // walk this (new) cur to the last node of the original list
    let mut cur = &mut res;
    while let Some(ref mut next) = cur.next {
        cur = next;
    }
    // link the last node of the original list to the head
    cur.next = Some(head);
    // return res, which is the head of the (now) rotated list
    Some(res)
}

fn append(ls: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in ls.iter().rev() {
        if head.is_none() {
            head = Some(Box::new(ListNode::new(*i)));
        } else {
            let node = Box::new(ListNode {
                val: *i,
                next: head.take()
            });
            head = Some(node);
        }
    }
    head
}

fn print(head: &Option<Box<ListNode>>) {
    let mut h = head;
    while let Some(node) = h.as_ref() {
        print!("{} ", node.val);
        h = &node.next;
    }
    println!();
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::{append, print, rotate_right};

    #[test]
    fn ex1() {
        let head = append(&[1,2,3,4,5]);
        let rotated = rotate_right(head, 2);
        print(&rotated);
    }

    #[test]
    fn ex2() {
        let head = append(&[1,2]);
        let rotated = rotate_right(head, 0);
        print(&rotated);
    }

}