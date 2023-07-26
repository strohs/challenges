package leetcode;


/**
 * Leetcode 161 - Intersection of two linked lists
 */
public class P161LLIntersection {

    private static class ListNode {
        int val;
        ListNode next;

        ListNode() {}

        ListNode(int x) {
            this.val = x;
            this.next = null;
        }

    }

    public ListNode getIntersectionNode(ListNode headA, ListNode headB) {
        ListNode cura = headA;
        ListNode nexta = cura.next;
        ListNode curb = headB;
        ListNode nextb = curb.next;


        while (true) {
            if (cura == null && curb != null) {
                cura = headA;
                curb = nextb;
                if (curb != null) nextb = curb.next;
            } else if (curb == null && cura != null) {
                curb = headB;
                cura = nexta;
                if (cura != null) nexta = cura.next;
            } else if (cura == null && curb == null) {
                break;
            } else if (cura == curb) {
                return cura;
            } else {
                cura = cura.next;
                curb = curb.next;
            }
        }

        return null;
    }

}
