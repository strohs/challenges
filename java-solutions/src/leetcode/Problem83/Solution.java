package leetcode.Problem83;

class Solution {

    static class ListNode {
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

    public ListNode deleteDuplicates(ListNode head) {
        if (head == null) {
            return head;
        }

        ListNode cur = head;
        ListNode next = cur.next;

        while (next != null) {
            if (cur.val != next.val) {
                cur.next = next;
                cur = next;
            }
            next = next.next;
        }
        if (cur.next != null) {
            cur.next = null;
        }
        return head;
    }

    public static void main(String[] args) {
        Solution sol = new Solution();
        ListNode head = new ListNode(1, new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(3, null)))));
        head = sol.deleteDuplicates(head);

        while (head != null) {
            System.out.println(head.val);
            head = head.next;
        }

        System.out.println();
        head = new ListNode(1, new ListNode(1, new ListNode(1, new ListNode(1, new ListNode(1, null)))));
        head = sol.deleteDuplicates(head);

        while (head != null) {
            System.out.println(head.val);
            head = head.next;
        }
    }
}
