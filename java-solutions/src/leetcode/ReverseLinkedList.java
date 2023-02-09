package leetcode;


/**
 * My attempt at reversing a linked list
 */
public class ReverseLinkedList {

    private static class ListNode {
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

    public static ListNode reverse(ListNode head) {
        ListNode prev = null;
        ListNode curr = head;
        ListNode next = head.next;

        while (curr != null) {
            curr.next = prev;
            prev = curr;
            curr = next;
            if (next != null) next = next.next;
        }
        return prev;
    }

    public static void printList(ListNode head) {
        while (head != null) {
            System.out.println(head.val);
            head = head.next;
        }
    }

    public static void main(String[] args) {
        // build a 3 node list
        ListNode list3 = new ListNode(1, new ListNode(2, new ListNode(3, null)));
        // build a 1 node list
        ListNode list1 = new ListNode(1, null);

        ListNode rev = ReverseLinkedList.reverse(list3);


        ReverseLinkedList.printList(rev);
    }
}
