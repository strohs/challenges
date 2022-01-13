package euler;

import java.time.Duration;
import java.time.Instant;
import java.util.ArrayList;
import java.util.List;

/**
 * Created with IntelliJ IDEA.
 * User: Cliff
 * Date: 3/16/2017
 * Time: 10:37 PM
 */
public class LotsOints {
    static final int MAX_INT = 100000000;


    public static void main( String[] args ) {
//        Instant start = Instant.now();
//
//        List ints = new ArrayList<Integer>( MAX_INT );
//        int i = 0;
//        while ( i < MAX_INT ) {
//            ints.add( i );
//            i++;
//        }
//        Instant fin = Instant.now();
//        System.out.println("DONE  " + Duration.between( start, fin ).toMillis() + "ms");
//        System.out.println(ints.size());

        primArray();
    }

    public static void primArray() {
        Instant start = Instant.now();

        int [] ints = new int[MAX_INT];
        int i = 0;
        while ( i < MAX_INT ) {
            ints[i] = i;
            i++;
        }
        Instant fin = Instant.now();
        System.out.println("prim DONE  " + Duration.between( start, fin ).toMillis() + "ms");
        System.out.println( ints.length );
    }
}
