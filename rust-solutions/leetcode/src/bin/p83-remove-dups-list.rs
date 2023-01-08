/// # Problem 83 - Remove Dups from a sorted linked list


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


pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut current = head.as_mut().unwrap();

    while let Some(next) = current.next.as_mut() {
        if current.val == next.val {
            // current == next, remove a duplicate by setting current.next to the next of next
            current.next = next.next.take();
        } else {
            // current != next, advance current to its next
            current = current.next.as_mut().unwrap();
        }
    }
    head
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::ListNode;

    fn build_list() -> Option<Box<ListNode>> {
        let mut head = None;
        // build in reverse
        for i in (1..=4).rev() {
            if head.is_none() {
                // new head
                head.replace(Box::new(ListNode::new(i)));
            } else {
                // appending to existing head
                let mut nn = Box::new(ListNode {
                    val: i,
                    next: head.take(),
                });
                head = Some(nn);
            }
        }
        head
    }

    #[test]
    fn test_mut_iter() {
        let mut head = build_list();
        let mut cn = head.as_mut().unwrap();
        let mut nn = cn.next.as_mut().unwrap();
        cn.val = 101;
        nn.val = 202;
        // try to leap frog
        // while let Some(cn) = n {
        //     println!("{}", &cn.val);
        //     h = cn;
        //     n = h.as_mut().map(|t| t.next.as_mut()).flatten();
        //
        //     // // if nn == cn loop until first != val
        //     // let mut next = cn.next.as_mut();
        //     // loop {
        //     //     match next {
        //     //         Some(nn) if cn.val == nn.val => next = nn.next.as_mut(),
        //     //         _ => break,
        //     //     }
        //     // }
        //     // // next will = the first node != cn  or None
        //     // cn.next = next.take();
        //     //
        //     // h = &mut cn.next;
        // }
    }
}