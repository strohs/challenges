package leetcode.Problem147;

import java.util.Arrays;
import java.util.Iterator;

/**
 * Leetcode 147 - Insertion Sort List
 * https://leetcode.com/problems/insertion-sort-list/
 */
public class Solution {

    public ListNode insertionSortList(ListNode head) {
        ListNode hp = head;

        while (hp != null) {
            ListNode cp = hp.next;
            if (cp != null && cp.val < hp.val) {
                // start at head of list and find insertion point
                ListNode pip = null;
                ListNode ip = head;
                while (cp.val > ip.val) {
                    pip = ip;
                    ip = ip.next;
                }
                // swap
                hp.next = cp.next;
                cp.next = ip;
                if (pip == null) {
                    head = cp;
                } else {
                    pip.next = cp;
                }
                hp = cp;
            }
            hp = hp.next;
        }
        return head;
    }

    private static ListNode buildList(int [] nums) {
        ListNode head = new ListNode();
        ListNode curr = head;
        Iterator<Integer> iter = Arrays.stream(nums).iterator();
        while (iter.hasNext()) {
            curr.val = iter.next();
            if (iter.hasNext()) {
                curr.next = new ListNode();
                curr = curr.next;
            } else {
                curr.next = null;
            }
        }
        return head;
    }

    static void printList(ListNode head) {
        while (head != null) {
            System.out.printf("%d ", head.val);
            head = head.next;
        }
        System.out.println();
    }

    public static void main(String[] args) {
        Solution sol = new Solution();
        int [] ex1 = new int[] { -1, 5, 3, 4, 0 };
        ListNode head = Solution.buildList(ex1);
        ListNode sorted = sol.insertionSortList(head);
        Solution.printList(sorted);
    }
}
