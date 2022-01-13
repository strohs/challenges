package euler;

import java.time.Duration;
import java.time.Instant;

/**
 * Created with IntelliJ IDEA.
 * User: Cliff
 * Date: 2/2/2016
 * Time: 12:55 PM
 */
public class Problem14 {

    public static void main( String[] args ) {
        int longest = 0;
        int terms = 0;
        int i;
        long j;
        //using this to keep track of runtime
        Instant start = Instant.now();

        for (i = 1; i <= 1000000; i++) {
            j = i;
            int this_terms = 1;
            while (j != 1) {
                this_terms++;
                if (this_terms > terms) {
                    terms = this_terms;
                    longest = i;
                }
                if (j % 2 == 0) {
                    j = j / 2;
                } else {
                    j = 3 * j + 1;
                }
            }
        }
        Instant end = Instant.now();
        String outf = String.format("longest: %d (%d)\n", longest, terms);
        System.out.println( outf );

        System.out.println("Runtime:" + Duration.between( start, end).toMillis() + "ms" );
    }
}
