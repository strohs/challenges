package google;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;

// merge two sorted lists into a new list
public class MergeSortedLists {

    // merge lists into a new list, and return it
    public List<Integer> mergeLists(List<Integer> ls, List<Integer> ms) {
        ArrayList<Integer> merged = new ArrayList<>();
        int lsi = 0;
        int msi = 0;
        while (lsi < ls.size() && msi < ms.size()) {
            if (ls.get(lsi) < ms.get(msi)) {
                merged.add(ls.get(lsi));
                lsi += 1;
            } else {
                merged.add(ms.get(msi));
                msi += 1;
            }
        }
        // if ls still has elements in it, add them to the merged list
        if (lsi < ls.size()) {
            while (lsi < ls.size()) {
                merged.add(ls.get(lsi++));
            }
        } else {
            // ms still has elements in it, add them to merged list
            while (msi < ms.size()) {
                merged.add(ms.get(msi++));
            }
        }
        return merged;
    }

    // combine ms into ls, mutating ls
    public void combineLists(List<Integer> ls, List<Integer> ms) {
        int lsi = 0;
        int msi = 0;
        while (lsi < ls.size() && msi < ms.size()) {
            if (ms.get(msi) < ls.get(lsi)) {
                ls.add(lsi, ms.get(msi));
                msi += 1;
            } else {
                lsi += 1;
            }
        }
        // if ms still has elements in it, add them to the merged list
        if (msi < ms.size()) {
            while (msi < ms.size()) {
                ls.add(ms.get(msi++));
            }
        }
    }

    public static void main(String[] args) {
        MergeSortedLists msl = new MergeSortedLists();
        List<Integer> l1 = new ArrayList<>(Arrays.asList(2,4,6,8,10));
        List<Integer> l2 = new ArrayList<>(Arrays.asList(1,3,5));
        msl.combineLists(l2, l1);
        System.out.println(l2);
    }
}
