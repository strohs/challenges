package other.graphs;

import java.util.HashSet;
import java.util.Set;

/**
 * Given a 2d grid of char that represents 'L' land, and 'W' water, return the number of
 * "islands"
 */
public class IslandCount {

    // a "wrapper" for a 2D row, col index in the island
    private record Index(int r, int c) {}

    private static char[][] islandGrid1 = {
            {'W', 'L', 'W', 'W', 'L', 'W'},
            {'L', 'L', 'W', 'W', 'L', 'W'},
            {'W', 'L', 'W', 'W', 'W', 'W'},
            {'W', 'W', 'W', 'L', 'L', 'W'},
            {'W', 'L', 'W', 'L', 'L', 'W'},
            {'W', 'W', 'W', 'W', 'W', 'W'},
    };

    public int islandCount(char[][] grid) {
        HashSet<Index> visited = new HashSet<>();
        int count = 0;

        for (int r = 0; r < grid.length; r++) {
            for (int c = 0; c < grid[0].length; c++) {
                if (explorePos(grid, r, c, visited)) {
                    count += 1;
                }
            }
        }
        return count;
    }

    // DFS the given r,c index. returns true if all Land, false if r,c is water or out of grid bounds
    boolean explorePos(char[][] grid, int r, int c, Set<Index> visited) {
        if (r < 0 || r >= grid.length || c < 0 || c >= grid[0].length) {
            return false;
        }
        if (grid[r][c] == 'W') {
            return false;
        }

        Index curIndex = new Index(r, c);
        if (visited.contains(curIndex)) {
            return false;
        }
        visited.add(curIndex);
        // explore neighbors of current position
        explorePos(grid, r-1, c, visited);
        explorePos(grid, r+1, c, visited);
        explorePos(grid, r, c+1, visited);
        explorePos(grid, r, c-1, visited);

        return true;
    }

    public static void main(String[] args) {
        IslandCount ic = new IslandCount();
        int count = ic.islandCount(IslandCount.islandGrid1);
        System.out.println("island count = " + count);
    }
}
