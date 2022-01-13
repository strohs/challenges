package leetcode;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class MergeSortGeneric {

    private <T> List<T> slice(List<T> arr, int start, int end) {
        return new ArrayList<>(arr.subList(start, end));
    }

    private <T extends Comparable<T>> List<T> combine(List<T> l, List<T> r) {
        List<T> combs = new ArrayList<>();
        int i = 0;
        int j = 0;

        while (i < l.size() && j < r.size()) {
            int comp = l.get(i).compareTo(r.get(j));
            if (comp <= 0) {
                combs.add(l.get(i));
                i++;
            } else {
                combs.add(r.get(j));
                j++;
            }
        }
        if (i < l.size())
            combs.addAll(l.subList(i, l.size()));
        if (j < r.size())
            combs.addAll(r.subList(j, r.size()));
        return combs;
    }

    public <T extends Comparable<T>> List<T> sort(List<T> ls) {
        if (ls.size() <= 1) {
            return ls;
        }
        int mp = ls.size() / 2;
        List<T> l = slice(ls, 0, mp);
        List<T> r = slice(ls, mp, ls.size());

        l = sort(l);
        r = sort(r);
        return combine(l, r);
    }

    public static void main(String[] args) {
        MergeSortGeneric ms = new MergeSortGeneric();

        List<Integer> ls1 = new ArrayList<>(Arrays.asList(9, 1, 3, 8, 7, 2, 7, 3, 6, 2, 5, 2, 4, 0, 5));
        List<Integer> sorted = ms.sort(ls1);
        System.out.println(sorted);

        List<Character> cs1 = Arrays.asList('c', 'a', 'z', 'g', 'y', 'b', 'w', 'e');
        List<Character> csorted = ms.sort(cs1);
        System.out.println(csorted);
    }
}
