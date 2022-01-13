/// # 19. Remove Nth Node from end of List
/// Given a linked list, remove the nth node from the end of the list and return its head
///
/// ## Example
/// Givem a list `1->2->3->4->5` and `n=2`
/// after removing the second node from the end, the list becomes 1->2->3->5
///
/// ## Note
/// `n` will always be a valid value


// Definition for singly-linked list.
function ListNode(val, next) {
    this.val = (val===undefined ? 0 : val)
    this.next = (next===undefined ? null : next)
}

// helper function to create a list of n nodes
function createList(n) {
    let head = new ListNode(n);
    for (let i=n-1; i > 0; i--) {
        head = new ListNode(i, head);
    }
    return head;
}

// prints each node of the list
function printList(head) {
    let node = head;
    while (node != null) {
        console.log(node.val);
        node = node.next;
    }
}

/**
 * @param {ListNode} head
 * @param {number} n
 * @return {ListNode}
 */
const removeNthFromEnd = function(head, n) {
    let tptr = head;    // tail pointer
    let i = 0;
    // increment tail pointer n times
    while (i <= n && tptr != null) {
        tptr = tptr.next;
        i++;
    }

    // increment head pointer and tail pointer until tail pointer reaches end of list
    let hptr = head;
    while (tptr != null) {
        hptr = hptr.next;
        tptr = tptr.next;
    }

    hptr.next = hptr.next.next;
};


let head = createList(5);
removeNthFromEnd(head, 2);
printList(head);
