package leetcode;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Problem 138 - Deep Copy linked list with random pointers
 * https://leetcode.com/problems/copy-list-with-random-pointer/
 *
 * This solution will use 2N memory with 2N runtime where N equals the size of the
 * input list
 */
public class P138CopyListRandomPointer {

    private static class Node {
        int val;
        Node next;
        Node random;

        public Node(int val) {
            this.val = val;
            this.next = null;
            this.random = null;
        }
    }

    public Node copyRandomList(Node head) {
        Map<Node, Integer> nodeMap = new HashMap<>();
        List<Node> copyList = new ArrayList<>();
        Node curHead = head;
        int i = 0;

        // step 1: copy the nodes of the original linked list into an ArrayList
        while (curHead != null) {
            nodeMap.put(curHead, i);
            copyList.add(new Node(curHead.val));
            i += 1;
            curHead = curHead.next;
        }

        // step 2: set up the next and random links of the copied list
        int j = 0;
        curHead = head;
        Node copyHead;
        while (j < i) {
            copyHead = copyList.get(j);
            if (j + 1 < i) {
                copyHead.next = copyList.get(j + 1);
            }
            if (curHead.random != null) {
                copyHead.random = copyList.get(nodeMap.get(curHead.random));
            }
            curHead = curHead.next;
            j += 1;
        }

        if (copyList.isEmpty())
            return null;
        else
            return copyList.get(0);
    }

    public static void main(String[] args) {
        P138CopyListRandomPointer solution = new P138CopyListRandomPointer();
    }
}
