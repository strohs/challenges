/// # 2. Add Two Numbers
/// You are given two non-empty linked lists representing two non-negative integers.
/// The digits are stored in reverse order, and each of their nodes contains a single digit.
/// Add the two numbers and return the sum as a linked list.
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
///
/// ## Constraints
/// - The number of nodes in each linked list is in the range `[1, 100]`
/// - `0 <= Node.val <= 9`
///

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

type Link = Option<Box<ListNode>>;

// append a new_node to the end of the linked list `ll`
fn append(head: &mut Link, val: i32) {
    let mut cur = head;

    while let Some(node) = cur {
        cur = &mut node.next;
    }
    cur.replace(Box::new(ListNode::new(val)));
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // parses the number into a "10s" digit and a "ones" digit
    let num_split = |n: i32| (n / 10, n % 10);

    let mut res_list: Link = None;
    let mut carry = 0;
    let mut l1_cur = &l1;
    let mut l2_cur = &l2;

    while l1_cur.is_some() || l2_cur.is_some() {
        let mut sum = 0;
        if let Some(l1_node) = &l1_cur {
            sum += l1_node.val;
            l1_cur = &l1_node.next;
        }
        if let Some(l2_node) = &l2_cur {
            sum += l2_node.val;
            l2_cur = &l2_node.next;
        }
        sum += carry;
        let (tens, ones) = num_split(sum);
        carry = tens;
        append(&mut res_list, ones);
    }
    // check if carry is > 0, we will need to add one more node
    if carry > 0 {
        append(&mut res_list, carry);
    }

    res_list
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Link;
    use super::ListNode;
    use crate::{add_two_numbers, append};

    #[test]
    fn test_num_split_over_10() {
        let num = 18;
        let (tens, ones) = (num / 10, num % 10);
        assert_eq!(tens, 1);
        assert_eq!(ones, 8);
    }

    #[test]
    fn test_num_split_equal_10() {
        let num = 10;
        let (tens, ones) = (num / 10, num % 10);
        assert_eq!(tens, 1);
        assert_eq!(ones, 0);
    }

    #[test]
    fn test_num_split_lessthan_10() {
        let num = 8;
        let (tens, ones) = (num / 10, num % 10);
        assert_eq!(tens, 0);
        assert_eq!(ones, 8);
    }

    #[test]
    fn create_list() {
        let l1: Link;
        let mut n1 = ListNode::new(1);
        let mut n2 = ListNode::new(2);
        let n3 = ListNode::new(3);
        n2.next = Some(Box::new(n3));
        n1.next = Some(Box::new(n2));
        l1 = Some(Box::new(n1));

        let mut cur_link = &l1;
        while let Some(cur_node) = cur_link {
            println!("cur_node val:{}", cur_node.val);
            cur_link = &cur_node.next;
        }
    }

    #[test]
    fn append_end_test() {
        let mut head: Option<Box<ListNode>> = None;
        append(&mut head, 11);
        append(&mut head, 22);
        append(&mut head, 33);
        append(&mut head, 44);
        dbg!(&head);
    }

    #[test]
    fn append_two_numbers_ex1() {
        let mut l1 = None;
        let mut l2 = None;

        append(&mut l1, 2);
        append(&mut l1, 4);
        append(&mut l1, 3);
        append(&mut l2, 5);
        append(&mut l2, 6);
        append(&mut l2, 4);

        let res = add_two_numbers(l1, l2);
        dbg!(&res);
    }

    #[test]
    fn append_two_numbers_ex2() {
        let mut l1 = None;
        let mut l2 = None;

        append(&mut l1, 0);
        append(&mut l2, 0);

        let res = add_two_numbers(l1, l2);
        dbg!(&res);
    }

    #[test]
    fn append_two_numbers_ex3() {
        let mut l1 = None;
        let mut l2 = None;

        append(&mut l1, 9);
        append(&mut l1, 9);
        append(&mut l1, 9);
        append(&mut l1, 9);
        append(&mut l1, 9);
        append(&mut l1, 9);
        append(&mut l1, 9);
        append(&mut l2, 9);
        append(&mut l2, 9);
        append(&mut l2, 9);
        append(&mut l2, 9);

        let res = add_two_numbers(l1, l2);
        dbg!(&res);
    }
}
