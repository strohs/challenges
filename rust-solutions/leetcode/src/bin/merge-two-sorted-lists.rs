/// 21. Merge Two Sorted Lists
/// Merge two sorted linked lists and return it as a new sorted list. The new list should be made
/// by splicing together the nodes of the first two lists.
///
/// ## Example
/// `Input: 1->2->4, 1->3->4`
/// `Output: 1->1->2->3->4->4`
///

type Link = Option<Box<ListNode>>;

// this is the Definition for the singly-linked list we must use
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Link,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// helper function that pushes a new node onto the head of the given linked list
pub fn push(head: &mut Link, elem: i32) {
    let mut new_node = Box::new(ListNode::new(elem));
    new_node.next = head.take();
    *head = Some(new_node);
}

fn merge(l1: &Link, l2: &Link) -> Link {
    let mut ml: Option<Box<ListNode>> = None;

    let mut cn1 = l1;
    let mut cn2 = l2;
    while cn1.is_some() || cn2.is_some() {
        match (cn1, cn2) {
            (Some(n1), Some(n2)) => {
                if n1.val <= n2.val {
                    push(&mut ml, n1.val);
                    push(&mut ml, n2.val);
                } else {
                    push(&mut ml, n2.val);
                    push(&mut ml, n1.val);
                }
                cn1 = &n1.next;
                cn2 = &n2.next;
            }
            (Some(n1), None) => {
                push(&mut ml, n1.val);
                cn1 = &n1.next;
            }
            (None, Some(n2)) => {
                push(&mut ml, n2.val);
                cn2 = &n2.next;
            }
            _ => (),
        }
    }

    ml
}

fn main() {
    let mut l1 = Some(Box::new(ListNode::new(4)));
    push(&mut l1, 2);
    push(&mut l1, 1);

    let mut l2 = Some(Box::new(ListNode::new(7)));
    push(&mut l2, 4);
    push(&mut l2, 3);
    push(&mut l2, 1);

    let merged_list = merge(&l1, &l2);
    dbg!(&merged_list);
    // let mut current = &l1;
    // while let Some(n) = current {
    //     dbg!(&n.val);
    //     current = &n.next;
    // }
}
