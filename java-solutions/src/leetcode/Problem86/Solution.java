package leetcode.Problem86;

/**
 * Problem 86 - Partition List
 * Given the head of a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.
 * You should preserve the original relative order of the nodes in each of the two partitions.
 * Example:
 * Given an x=3
 * 1 -> 4 -> 3 -> 2 -> 5 -> 2
 * becomes
 * 1 -> 2 -> 2 -> 4 -> 3 -> 5
 */
public class Solution {

    static class ListNode {
        int val;
        leetcode.Problem86.Solution.ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, leetcode.Problem86.Solution.ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    // 'rh' is the "right hand list", which will point to nodes >= to the partition value
    // rhh points to the head od the 'right hand list'
    private ListNode rhh = null;
    // rtt points to the tail node of the right hand list
    private ListNode rht = null;

    // appends the given node to the end of the right hand list
    private void appendToRightList(ListNode node) {
        if (this.rhh == null) {
            this.rhh = new ListNode(node.val, null);
            this.rht = this.rhh;
        } else {
            this.rht.next = node;
            this.rht = node;
        }
    }

    public ListNode partition(ListNode head, int x) {
        if (head == null) {
            return null;
        }

        // keep track of cur and a cur.next node
        ListNode cur = head;
        // prev points to the node before cur
        ListNode prev = null;

        while (cur != null) {
            if (cur.val < x) {
                prev = cur;
                cur = cur.next;
            } else {
                if (prev == null) {
                    // at beginning of list
                    ListNode split = cur;
                    cur = cur.next;
                    head = cur;
                    split.next = null;
                    appendToRightList(split);
                } else {
                    prev.next = cur.next;
                    cur.next = null;
                    appendToRightList(cur);
                    cur = prev.next;
                }

            }
        }
        if (prev == null) {
            head = this.rhh;
        } else {
            prev.next = this.rhh;
        }

        return head;
    }



    public static void main(String[] args) {
        Solution sol = new Solution();
        Solution.ListNode head = new ListNode(1, new ListNode(3, null));
        //head = sol.partition(head, 3);

        while (head != null) {
            System.out.println(head.val);
            head = head.next;
        }

        System.out.println("-----");

        sol = new Solution();
        head = new ListNode(3, new ListNode(1, null));
        head = sol.partition(head, 3);

        while (head != null) {
            System.out.println(head.val);
            head = head.next;
        }

        System.out.println("-----");

        sol = new Solution();
        head = new ListNode(3,null);
        head = sol.partition(head, 3);

        while (head != null) {
            System.out.println(head.val);
            head = head.next;
        }

        System.out.println("-----");

        sol = new Solution();
        head = new ListNode(1,null);
        head = sol.partition(head, 1);

        while (head != null) {
            System.out.println(head.val);
            head = head.next;
        }

        System.out.println("-----");

        sol = new Solution();
        head = new ListNode(1, new ListNode(4, new ListNode(3, new ListNode( 2, new ListNode(5, new ListNode(2, null))))));
        head = sol.partition(head, 3);

        while (head != null) {
            System.out.println(head.val);
            head = head.next;
        }

        System.out.println("-----");

        sol = new Solution();
        head = new ListNode(3, new ListNode(3, new ListNode(3, new ListNode( 3, new ListNode(3, new ListNode(3, null))))));
        head = sol.partition(head, 3);

        while (head != null) {
            System.out.println(head.val);
            head = head.next;
        }

        System.out.println("-----");

        sol = new Solution();
        head = new ListNode(1, new ListNode(1, new ListNode(1, new ListNode( 1,  null))));
        head = sol.partition(head, 3);

        while (head != null) {
            System.out.println(head.val);
            head = head.next;
        }
    }




}
