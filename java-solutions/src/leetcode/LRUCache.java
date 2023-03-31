package leetcode;

import java.util.LinkedHashMap;
import java.util.Optional;

/**
 * Problem 146 - LRU Cache
 * https://leetcode.com/problems/lru-cache/
 * This example uses Java's LinkedHashMap to maintain the elements in LRU order. This means
 * that the first element of the KeySet will be the LRU element and the last element will be the
 * last accessed.
 */
public class LRUCache {

    private int capacity = 0;
    private LinkedHashMap<Integer, Integer> cache;

    public LRUCache(int capacity) {
        this.capacity = capacity;
        this.cache = new LinkedHashMap<>(capacity, 1.0f, true);
    }

    public int get(int key) {
        return this.cache.getOrDefault(key, -1);
    }

    public void put(int key, int value) {
        if (this.cache.size() == capacity && !this.cache.containsKey(key)) {
            // remove first element before putting new element
            // first will be the LRU element
            Optional<Integer> first = this.cache.keySet().stream().findFirst();
            first.ifPresent(integer -> this.cache.remove(integer));
        }
        this.cache.put(key, value);
    }

}
