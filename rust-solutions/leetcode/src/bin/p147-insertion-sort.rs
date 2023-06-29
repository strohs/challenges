/// # Leetcode - 147 insertion sort
///

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

struct Solution {}

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut head| {
            let mut curr = head.as_mut() as *mut ListNode;
            unsafe {
                while let Some(curr_next) = (*curr).next.as_mut() {
                    if curr_next.val < head.val {
                        std::mem::swap(&mut curr_next.val, &mut head.val);
                    }
                    curr = curr_next.as_mut() as *mut ListNode;
                }
            }
            Box::new(ListNode {
                val: head.val,
                next: Solution::insertion_sort_list(head.next),
            })
        })
    }


    fn print_list(head: &Option<Box<ListNode>>) {
        let mut cur = head;
        while let Some(node) = cur {
            print!("{} ", node.val);
            cur = &node.next;
        }
        println!();
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::Solution;
    use super::ListNode;


    fn build_list() -> Option<Box<ListNode>> {
        // -1,5,3,4,0
        let mut head = Some(Box::new(ListNode::new(-1)));
        let mut five = Some(Box::new(ListNode::new(5)));
        let mut three = Some(Box::new(ListNode::new(3)));
        let mut four = Some(Box::new(ListNode::new(4)));
        let zero = Some(Box::new(ListNode::new(0)));
        four.as_mut().map(|n| n.next = zero);
        three.as_mut().map(|n| n.next = four);
        five.as_mut().map(|n| n.next = three);
        head.as_mut().map(|n| n.next = five);
        head
    }

    #[test]
    fn build() {
        let l = build_list();
        Solution::print_list(&l);
        assert!(l.is_some());
    }

    #[test]
    fn test_advance() {
        let mut head = build_list();

        //Solution::print_list(nn.unwrap());

        assert!(head.is_some());
    }

}