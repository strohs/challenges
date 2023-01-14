use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

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



pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    /// NodeRec exists since we cant modify ListNode to implement Ord
    #[derive(Debug)]
    struct NodeRec<'a>(i32, Option<&'a Box<ListNode>>);
    impl<'a> Eq for NodeRec<'a> {}
    impl<'a> PartialEq<Self> for NodeRec<'a> {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }
    impl<'a> PartialOrd<Self> for NodeRec<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<'a> Ord for NodeRec<'a> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    let mut sorted: Option<Box<ListNode>> = None;
    let mut tail = sorted.as_mut();
    let mut heap = BinaryHeap::new();

    // initially populate the heap with the first element from each list
    for list in lists.iter() {
        if let Some(node) = list {
            heap.push(Reverse(NodeRec(node.val, node.next.as_ref())));
        }
    }

    // now push and pop elements from the lists into the heap until heap is empty
    while !heap.is_empty() {
        let NodeRec(val, next_node) = heap.pop().unwrap().0;
        // append val to the sorted list
        if let Some(t) = tail {
            t.next = Some(Box::new(ListNode::new(val)));
            tail = t.next.as_mut();
        } else {
            // create new list
            sorted = Some(Box::new(ListNode::new(val)));
            tail = sorted.as_mut();
        }

        if let Some(n) = next_node {
            heap.push(Reverse(NodeRec(n.val,  n.next.as_ref())));
        }
    }

    sorted
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{ListNode, merge_k_lists};

    fn build_list(input: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        // build in reverse
        for i in input.into_iter().rev() {
            if head.is_none() {
                // new head
                head.replace(Box::new(ListNode::new(i)));
            } else {
                // appending to existing head
                let nn = Box::new(ListNode {
                    val: i,
                    next: head.take(),
                });
                head = Some(nn);
            }
        }
        head
    }

    #[test]
    fn test_empty() {
        let list1 = build_list(vec![]);
        let list2 = build_list(vec![]);
        let lists = vec![list1, list2];
        let sorted = merge_k_lists(lists);
        assert_eq!(sorted, None);
    }

    #[test]
    fn test_one_list() {
        let list1 = build_list(vec![1]);
        let list2 = build_list(vec![]);
        let lists = vec![list1, list2];
        let sorted = merge_k_lists(lists);
        dbg!(sorted); // 1
    }

    #[test]
    fn test_two_lists() {
        let list1 = build_list(vec![5]);
        let list2 = build_list(vec![1]);
        let lists = vec![list1, list2];
        let sorted = merge_k_lists(lists);
        dbg!(sorted); // 1, 5
    }

    #[test]
    fn tests() {
        let list1 = build_list(vec![12,14,16,18]);
        let list2 = build_list(vec![1,3,5]);
        let list3 = build_list(vec![1,2,4,100]);
        let lists = vec![list1, list2, list3];
        let sorted = merge_k_lists(lists);
        dbg!(sorted); // 1,1,2,3,4,5,12,14,16,18,100
    }
}