package google;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Random;

/**
 * A sample data structure from a mock coding interview that needs to perform the following operations
 * - insert a value
 * - remove a value
 * - get a random value out of the store, i,e, remove and return a random value
 * In our case, the value can be Integer
 * The goal is for all three of these methods to perform in constant time. As such, this class will use a HashMap and an ArrayList
 * together to store the values. The HashMap will map the value to its index in the ArrayList,
 * the ArrayList will also store the value (at the correct index). The ArrayList is used to speed up the getRandom method.
 */
public class Store {

    private final HashMap<Integer, Integer> map;

    private final ArrayList<Integer> list;

    private final Random rand;

    public Store() {
        this.map = new HashMap<>();
        this.list = new ArrayList<>();
        this.rand = new Random();
    }

    public void insert(int n) {
        this.list.add(n);
        this.map.put(n, this.list.size() - 1);
    }

    // returns true if element was deleted, false if element did not exist in the Store
    public boolean remove(int n) {
        // remove the oldValue from the map which will return it's current index in the list, oldIndex
        // get (remove) the value currently stored at the last index in the list, lastVal
        // associate lastVal with oldIndex in the map
        // set oldIndex = to lastVal in the list
        Integer oldIndex = this.map.remove(n);
        if (oldIndex != null) {
            Integer lastVal = this.list.remove(this.list.size() - 1);
            this.map.put(lastVal, oldIndex);
            this.list.set(oldIndex, lastVal);
            return true;
        } else {
            return false;
        }
    }

    public Integer getRandom() {
        int randIndex = this.rand.nextInt(this.list.size());
        return this.map.get(this.list.get(randIndex));
    }
}
