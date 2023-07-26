package leetcode;

/**
 * Leetcode 141 - Linked List Cycle
 * https://leetcode.com/problems/linked-list-cycle/
 * this solution used a tortoise and hare solution
 */
public class P141LinkedListCycle {

    private class ListNode {
        int val;
        ListNode next;

        ListNode() {}

        ListNode(int x) {
            this.val = x;
            this.next = null;
        }

    }

    public boolean hasCycle(ListNode head) {
        if (head == null) return false;
        ListNode n = head;
        ListNode nn = head.next;

        while (n != null && nn != null) {
            if (n == nn) {
                return true;
            }
            n = n.next;
            nn = nextNext(nn);
        }
        return false;
    }

    private ListNode nextNext(ListNode node) {
        ListNode nn = null;
        if (node != null) {
            nn = node.next;
            if (nn != null) {
                nn = nn.next;
            }
        }
        return nn;
    }

}
