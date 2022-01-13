/// # 19. Remove Nth Node from end of List
/// Given a linked list, remove the nth node from the end of the list and return its head
///
/// ## Example
/// Givem a list `1->2->3->4->5` and `n=2`
/// after removing the second node from the end, the list becomes 1->2->3->5
///
/// ## Note
/// `n` will always be a valid value

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

/// helper function that pushes a new node onto the head of this linked list
pub fn push(head: &mut Link, elem: i32) {
    let mut new_node = Box::new(ListNode::new(elem));
    new_node.next = head.take();
    *head = Some(new_node);
}

fn remove_nth_from_end(list: &mut Link, n: usize) {
    if list.is_none() {
        return;
    }

    // compute list length O(n)
    let get_length = |l: &Link| {
        let mut length = 0;
        let mut current = l;
        while let Some(n) = current {
            length += 1;
            current = &n.next;
        }
        length
    };

    let length = get_length(list);
    let mut i = 0;
    let mut current = list;
    while i < length - n {
        if let Some(link) = current {
            current = &mut link.next;
        } else {
            panic!("Invalid node!");
        }
        i += 1;
    }
    *current = current.as_mut().unwrap().next.take();
}

fn main() {
    let mut h = Some(Box::new(ListNode::new(5)));
    push(&mut h, 4);
    push(&mut h, 3);
    push(&mut h, 2);
    push(&mut h, 1);

    //dbg!(&h);

    remove_nth_from_end(&mut h, 2);

    dbg!(&h);
}
