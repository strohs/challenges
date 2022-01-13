package leetcode;/// # 24 Swap Nodes in Pairs
/// Given a linked list, swap every two adjacent nodes and return its head.
/// You may not modify the values in the list's nodes. Only nodes itself may be changed.
///
/// Constraints:
/// - the number of nodes in the list is in the range [0, 100]
/// - 0 <= Node.val <= 100
///
/// ## Example 1
/// Input: head = [1, 2, 3, 4]
/// Output: [2, 1, 4, 3]
///
/// ## Example 2
/// Input: head = []
/// Output: []

public class SwapNodesInPairs {

    class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }

    }

    // helper function that returns a List of ListNodes using the values in arr, which is an Array of numbers
    ListNode buildList(int[] arr) {
        var head = new ListNode();
        var cur_node = head;
        for (int n : arr) {
            var node = new ListNode(n);
            cur_node.next = node;
            cur_node = node;
        }
        return head;
    }

    // returns true if node is not null, AND node.next is NOT null
    static boolean hasNext(ListNode node) {
        if (node != null && node.next != null)
            return true;
        else
            return false;
    }

    // prints the value of each node to the console.log
    static void printList(ListNode head) {
        var cur_node = head;
        while (cur_node != null) {
            System.out.println(cur_node.val);
            cur_node = cur_node.next;
        }
    }

    // swap pairs of nodes in the linked list pointed to by head
    public ListNode swapPairs(ListNode head) {
        var cur_node = head;

        while (cur_node != null) {
            // get next two nodes
            var n1 = hasNext(cur_node) ? cur_node.next : null;
            var n2 = hasNext(n1) ? n1.next : null;

            // swap n1 and n2
            if (n1 != null && n2 != null) {
                n1.next = n2.next;
                n2.next = n1;
                cur_node.next = n2;
            }
            // advance cur_node by two nodes
            cur_node = cur_node.next;
            if (hasNext(cur_node)) cur_node = cur_node.next;
        }

        return head;
    }

    ;

    public static void main(String[] args) {
        SwapNodesInPairs swapper = new SwapNodesInPairs();
        var head = swapper.buildList(new int[]{1, 2, 3, 4});
        var swapped = swapper.swapPairs(head);
        SwapNodesInPairs.printList(swapped);
    }
}
