package google;

import java.util.*;

/// # Remove-Islands (medium coding challenge)
/// You are given a 2d matrix consisting of 1's and 0's.
/// 1 indicates a "black" pixel and 0 indicates a "white" pixel.
/// Your task is to remove all black colored pixels that are **NOT** connected to the border
/// of the matrix, by replacing them with 0.
/// You should return the (modified) input matrix
///
/// ## Example
/// Given the following matrix
/// ```
/// let mut m1: [[u8; 6]; 6] = [
///         [1, 0, 0, 0, 0, 0],
///         [0, 1, 0, 1, 1, 1],
///         [0, 0, 1, 0, 1, 0],
///         [1, 1, 0, 0, 1, 0],
///         [1, 0, 1, 1, 0, 0],
///         [1, 0, 0, 0, 0, 1],
///     ];
/// ```
/// the cells at (1,1) (2,2), (4,2) and (4,3) should be replaced with 0
///
/// ## Output
/// return the input matrix but with the NON-connected black pixels replaced with a 0 (white) pixel


public class RemoveIslands {

    /**
     * Hold the row and col index of a "node" within a 2D matrix
     */
    private record NodeIndex(int r, int c) {
    }


    /**
     * returns NodeIndex(s) that are neighbors of the Node at index r,c in the given matrix: m
     * @param ni - r,c index of the node
     * @param m - a 2D matrix
     * @return a List of NodeIndices containing the neighbors
     */
    List<NodeIndex> neighbors(NodeIndex ni, int[][] m) {
        var ns = new ArrayList<NodeIndex>();

        // up neighbor
        if (ni.r > 0) {
            ns.add(new NodeIndex(ni.r-1, ni.c));
        }
        // down neighbor
        if (ni.r < m.length - 1) {
            ns.add(new NodeIndex(ni.r+1, ni.c));
        }
        // left neighbor
        if (ni.c > 0) {
            ns.add(new NodeIndex(ni.r, ni.c - 1));
        }
        // right neighbor
        if (ni.c < m[0].length - 1) {
            ns.add(new NodeIndex(ni.r, ni.c + 1));
        }

        return ns;
    }

    /**
     * returns true if the given NodeIndex is on the border of the matrix: m
     * @param ni
     * @param m
     * @return true if ni is a border node, else false
     */
    boolean isBorderNode(NodeIndex ni, int [][] m) {
        return ni.r == 0 || ni.r == m.length - 1 || ni.c == 0 || ni.c == m[0].length - 1;
    }

    /**
     * Iterates through all nodes of the given matrix and creates a "connectionMap" that maps
     * "1" nodes on the border, as well as their connected "1" node neighbors, to true
     * @param m
     * @return
     */
    Map<NodeIndex, Boolean> buildConnMap(int[][] m) {
        // the ConnectionMap keeps track of all nodes that are connected to a border node with value of "1"
        var cm = new HashMap<NodeIndex, Boolean>();

        for (int r = 0; r < m.length; r++) {
            for (int c = 0; c < m[0].length; c++) {
                NodeIndex curNode = new NodeIndex(r, c);

                // if curNode is a border node AND it has not yet been visited, perform a depth first search
                // to determine which other "1" nodes are connected to the border node and store them in the
                // connection map
                if (m[r][c] == 1 && isBorderNode(curNode, m) && !cm.containsKey(curNode)) {
                    Stack<NodeIndex> toVisit = new Stack<>();
                    toVisit.add(curNode);

                    while (!toVisit.isEmpty()) {
                        NodeIndex ni = toVisit.pop();
                        cm.put(ni, true);
                        List<NodeIndex> nbrs = neighbors(ni, m)
                                .stream()
                                .filter( n -> m[n.r][n.c] == 1 && !cm.containsKey(n))
                                .toList();
                        toVisit.addAll(nbrs);
                    }
                }
            }
        }
        return cm;
    }

    public static void main(String[] args) {

        int[][] m1 = new int[][]{
                {1, 0, 0, 0, 0, 0},
                {0, 1, 0, 1, 1, 1},
                {0, 0, 1, 0, 1, 0},
                {1, 1, 0, 0, 1, 0},
                {1, 0, 1, 1, 0, 0},
                {1, 0, 0, 0, 0, 1},
        };

        var ri = new RemoveIslands();

        var connMap = ri.buildConnMap(m1);

        // print the nodes that are not connected to the border
        for (int r = 0; r < m1.length; r++) {
            for (int c = 0; c < m1[0].length; c++) {
                if (m1[r][c] == 1 && !connMap.containsKey(new NodeIndex(r,c))) {
                    System.out.printf("node not connected %d:%d%n", r, c);
                    m1[r][c] = 0;
                }
            }
        }
        // print the modified matrix
        for (int[] row: m1 ) {
            System.out.println(Arrays.toString(row));
        }

    }
}
