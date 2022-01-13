package euler;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

/**
 * Maximum Path Sum I
 *
 * By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total
 * from top to bottom is 23.
 *
 *    3
 *   7 4
 *  2 4 6
 * 8 5 9 3
 *
 * That is, 3 + 7 + 4 + 9 = 23.
 *
 * The input for this challenge is in the file: p18-input.txt
 *
 * Solution Details:
 * Rather than starting from the root and trying to brute force all possible paths to the bottom of the tree, this
 * solution will start from the bottom row - 1, compute the max sum for each node, and store the max sum in that node. As the
 * algorithm works towards the root, each node will contain the max sum of the nodes below it.
 * The root node will contain the maximum sum path once the algorithm is finished
 * 
 * User: Cliff
 */
public class Problem18 {
    static final String inputFile = "./euler-java/src/p18-input.txt";

    //builds a triangular array from the triangle data stored in a file
    private Triangle build( String inputPath ) {
        //temp ArrayList to hold values
        List<List<Integer>> list = new ArrayList<>();

        //read file into stream, try-with-resources
        try ( Stream<String> stream = Files.lines( Paths.get(inputPath) ) ) {
            list = stream
                    .map( line -> Arrays.stream( line.split( " " ) )      //split file line into list of integer strings
                            .map( Integer::parseInt )                           //parse each integer string into an Integer
                            .collect(Collectors.toList()) )                     //collect the Integers into a List<Integer>
                    .collect( Collectors.toList() );                            //collect the List<Integer> into a List<List<Integer>>
        } catch (IOException e) {
            e.printStackTrace();
        }
        Triangle triangle = new Triangle();
        triangle.rows = list.size();
        triangle.cols = list.get( list.size() - 1 ).size();
        triangle.array = list.stream().map( u -> u.toArray(new Integer[0])).toArray(Integer[][]::new);
        return triangle;
    }

    //Data class that stores the input triangle as a triangular array
    private class Triangle {
        private int rows;
        private int cols;
        private Integer [][] array;

        @Override
        public String toString() {
            
            final StringBuilder sb = new StringBuilder();
            //sb.append( String.format( "%s X %s \n",rows,cols ) );
            Arrays.stream( array )
                    .map( row -> {
                        String rowStr = Arrays.stream( row )
                                .map( integer -> String.format( "%03d",integer ) )
                                .collect( Collectors.joining( " " ) );
                        return leftPad( cols, row.length, rowStr );
                    })
                    .forEach( s -> sb.append( s ).append( "\n" ) );
            return sb.toString();
        }

        //left-pad 'str' with spaces, so that the triangle is pretty printed
        private String leftPad( int maxElems, int rowElems, String str ) {
            int padding = (int) ( ( (maxElems - rowElems) / 2.0 ) * 4 );
            if ( padding <= 0 )
                padding = 0;

            StringBuilder sb = new StringBuilder();
            for (int i = 0; i < padding; i++ )
                sb.append( " " );
            return sb.append( str ).toString();
        }
    }
    
    private int leftChild( Triangle tri,  int row, int col ) {
        //left child will be in the next row, at same col index as current col index
        return tri.array[ row + 1][col];
    }

    private int rightChild( Triangle tri,  int row, int col ) {
        //right child will be in the next row, at curCol index + 1
        return tri.array[ row + 1][col + 1];
    }

    //get current cell + left child
    private int leftSum( Triangle tri, int row, int col ) {
        return tri.array[row][col] + leftChild( tri, row, col );
    }

    //get current cell + right child
    private int rightSum( Triangle tri, int row, int col ) {
        return tri.array[row][col] + rightChild( tri, row, col );
    }

    //return the maximum sum between the current cell + left child OR current cell + right child
    private int maxSum( Triangle tri, int row, int col ) {
        return Math.max( leftSum( tri, row, col ), rightSum( tri,row,col ) );
    }

    //start at bottom of triangle and work up to the top, keeping track of the max sum for each cell in the matrix
    public long findMaxPathSum( String inputFile ) {
        Triangle tri = build( inputFile );
        System.out.println( tri );

        //int curRow = tri.rows - 2; //start at second to last row
        //int maxSum = 0;
        //        while (curRow >= 0 ) {
//            for( int curCol = 0; curCol < tri.array[curRow].length; curCol++ ) {
//                maxSum = maxSum( tri, curRow, curCol );
//                tri.array[curRow][curCol] = maxSum;
//                debug( tri, curRow, curCol );
//            }
//            curRow--;
//        }

        IntStream rowIdxStream = revRange( 0, tri.rows - 1 );
        rowIdxStream.forEach( rowIdx -> {
            IntStream colIdxStream = IntStream.range( 0, tri.array[rowIdx].length );
            colIdxStream.forEach( colIdx -> tri.array[rowIdx][colIdx] = maxSum(tri, rowIdx, colIdx) );
        } );

        debug( tri,0,0 );
        return tri.array[0][0];
    }

    private void debug( Triangle t, int r, int c) {
        System.out.println( String.format( "%s x %s --------------------------------------------------",r,c ) );
        System.out.println( t.toString() );
    }

    //returns an IntStream in reverse order
    static IntStream revRange(int from, int to) {
        return IntStream.range(from, to)
                .map(i -> to - i + from - 1);
    }

    public static void main( String[] args ) {
        Problem18 p18 = new Problem18();
        long maxSum = p18.findMaxPathSum( inputFile );
        System.out.println("Max Sum is ==>" + maxSum);
    }
}
