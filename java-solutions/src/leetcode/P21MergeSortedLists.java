package leetcode;

/**
 * Merge two sorted linked lists and return it as a sorted list. The list should be made by splicing together the
 * nodes of the first two lists.
 *
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
public class P21MergeSortedLists {

    public ListNode mergeTwoLists(ListNode l1, ListNode l2) {
        ListNode head = new ListNode();

        if (l1 == null && l2 == null) {
            return null;
        } else if (l1 == null) {
            head.val = l2.val;
            l2 = l2.next;
        } else if (l2 == null) {
            head.val = l1.val;
            l1 = l1.next;
        } else {
            if (l1.val < l2.val) {
                head.val = l1.val;
                l1 = l1.next;
            } else {
                head.val = l2.val;
                l2 = l2.next;
            }
        }

        ListNode cur = head;
        while (l1 != null || l2 != null) {

            if (l1 == null) {
                cur.next = new ListNode(l2.val);
                l2 = l2.next;
            } else if (l2 == null) {
                cur.next = new ListNode(l1.val);
                l1 = l1.next;
            } else {
                if (l1.val < l2.val) {
                    cur.next = new ListNode(l1.val);
                    l1 = l1.next;
                } else {
                    cur.next = new ListNode(l2.val);
                    l2 = l2.next;
                }
            }
            cur = cur.next;
        }

        return head;
    }

    public static void main(String[] args) {
        //var l1 = new ListNode(0, new ListNode(2, new ListNode(4)));
        //var l2 = new ListNode(1, new ListNode(3, new ListNode(6)));

        var l1 = new ListNode(-1, new ListNode(2));
        var l2 = new ListNode(1, new ListNode(3, new ListNode(6)));

        var p21 = new P21MergeSortedLists();
        var merged = p21.mergeTwoLists(l1, l2);
        while (merged != null) {
            System.out.println(merged.val);
            merged = merged.next;
        }
    }
}
