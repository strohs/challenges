# Given an array of n elements, where each element is at most k away from its
# target position, devise an algorithm that sorts in O(n log k) time. For example,
# let us consider k is 2, an element at index 7 in the sorted array, can be at
# indexes 5, 6, 7, 8, 9 in the given array
# # Example
# ```
# Input : arr[] = {6, 5, 3, 2, 8, 10, 9}
#             k = 3
# Output : arr[] = {2, 3, 5, 6, 8, 9, 10}
# ```
from heapq import heappush, heappop, heapreplace


def sort_nearly_sorted(arr: [int], k: int):
    heap = []
    for i in range(len(arr)):
        if i >= k + 1:
            smallest = heapreplace(heap, arr[i])
            arr[i - (k + 1)] = smallest
        else:
            heappush(heap, arr[i])

    # now push remaining k+1 items from the heap into the array
    for i in range(len(arr), len(arr) + k + 1):
        arr[i - (k + 1)] = heappop(heap)


arr1 = [6, 5, 3, 2, 8, 10, 8, 9]
sort_nearly_sorted(arr1, 3)
print(arr1)
