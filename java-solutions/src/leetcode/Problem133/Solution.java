package leetcode.Problem133;

import java.util.ArrayList;
import java.util.List;
import java.util.HashMap;

/**
 * Leetcode Problem 133 - Clone Graph
 * https://leetcode.com/problems/clone-graph/
 */
class Solution {

    public Node cloneGraph(Node node) {
        if (node == null) return new Node();

        List<Integer> visited = new ArrayList<>();
        List<Node> toVisit = new ArrayList<>();
        HashMap<Integer, Node> nmap = new HashMap<>();
        toVisit.add(node);

        while (!toVisit.isEmpty()) {
            Node curr = toVisit.remove(0);
            if (!visited.contains(curr.val)) {
                visited.add(curr.val);
                Node cloned_node = nmap.getOrDefault(curr.val, new Node(curr.val));
                for (Node n : curr.neighbors) {
                    Node neighbor_node = nmap.getOrDefault(n.val, new Node(n.val));
                    if (!cloned_node.neighbors.contains(neighbor_node)) {
                        cloned_node.neighbors.add(neighbor_node);
                    }
                    nmap.put(n.val, neighbor_node);
                    toVisit.add(n);
                }
                nmap.put(cloned_node.val, cloned_node);
            }

        }

        return nmap.get(1);
    }

    public static void main(String[] args) {
        Solution sol = new Solution();
        Node n1 = new Node(1);
        Node n2 = new Node(2);
        Node n3 = new Node(3);
        Node n4 = new Node(4);
        n1.neighbors.add(n4);
        n1.neighbors.add(n2);
        n2.neighbors.add(n1);
        n2.neighbors.add(n3);
        n3.neighbors.add(n2);
        n3.neighbors.add(n4);
        n4.neighbors.add(n1);
        n4.neighbors.add(n3);

        Node clone = sol.cloneGraph(n1);
        sol.printGraph(clone);
        System.out.println("---------------");
        n1 = new Node(1);
        clone = sol.cloneGraph(n1);
        sol.printGraph(clone);
        System.out.println("---------------");
        n1 = null;
        clone = sol.cloneGraph(n1);
        sol.printGraph(clone);
    }

    private void printGraph(Node root) {
        List<Integer> visited = new ArrayList<>();
        List<Node> toVisit = new ArrayList<>();
        toVisit.add(root);
        while (!toVisit.isEmpty()) {
            Node c = toVisit.remove(0);
            if (!visited.contains(c.val)) {
                visited.add(c.val);
                System.out.print(c.val + "->");
                for (Node nn : c.neighbors) {
                    System.out.print(" " + nn.val);
                    toVisit.add(nn);
                }
                System.out.println();
            }
        }
    }
}
