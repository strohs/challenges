/// NOT Complete

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

pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let (mut lhead, mut hhead) = (None, None);
    let (mut low, mut high) = (&mut lhead, &mut hhead);
    while let Some(mut node) = head {
        head = node.next.take();
        if node.val < x {
            *low = Some(node);
            low = &mut low.as_deref_mut().unwrap().next;
        } else {
            *high = Some(node);
            high = &mut high.as_deref_mut().unwrap().next;
        }
    }
    *low = hhead;
    lhead
}


fn main() {
    let mut l1 = build_list(vec![1, 4, 3, 2, 5, 2]);
    let x = 3;
    let mut cur = l1.as_mut().unwrap();
    while let Some(next) = cur.next.as_mut() {
        if cur.val >= x {
            println!("{} is >= {}", cur.val, x);
        } else {
            println!("{} is < {}", cur.val, x);
        }
        cur = next;
    }
}