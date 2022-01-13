package hackerrank.algos.easy;

import java.util.Arrays;
import java.util.List;
import java.util.function.Predicate;
import java.util.stream.IntStream;

public class BonAppetit {

    private static Predicate<String> capitalized() {
        return s -> s.startsWith( s.substring(0,1).toUpperCase() );
    }

    static void bonAppetit(List<Integer> bill, int k, int b) {
        int actualAmt = IntStream
                .range(0, bill.size())
                .filter( i -> i != k)
                .mapToObj(bill::get)
                .reduce(Integer::sum).get();
        actualAmt = actualAmt / 2;
        if ( actualAmt == b ) System.out.println("Bon Appetit");
        else System.out.println( b - actualAmt );
    }

    public static void main(String[] args) {
        List<Integer> bill = Arrays.asList(3,10,2,9);
        bonAppetit( bill, 1, 12);

    }
}
