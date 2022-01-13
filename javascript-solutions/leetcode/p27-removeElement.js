/// # 27 Remove Element
/// Given an array nums and a value val, remove all instances of that value in-place and return the
/// new length.
/// Do not allocate extra space for another array, you must do this by modifying the input array
/// in-place with O(1) extra memory.
/// The order of elements can be changed. It doesn't matter what you leave beyond the new length.
///
/// ## Example 1
/// Given nums = `[3,2,2,3], val = 3`,
/// our function should return length = 2, with the first two elements of nums being 2.
/// It doesn't matter what you leave beyond the returned length.
///
/// ## Example 2
/// Given nums = `[0,1,2,2,3,0,4,2], val = 2`,
/// Your function should return length = 5, with the first five elements of nums containing
/// 0, 1, 3, 0, and 4.
/// Note that the order of those five elements can be arbitrary.
///
/// Constraints
/// 0 <= nums.length <= 100
/// 0 <= nums[i] <= 50
/// 0 <= val <= 100




/**
 * @param {number[]} nums
 * @param {number} val
 * @return {number}
 */
var removeElement = function(nums, val) {

    // swap two elements in an array
    function swap(arr, i1, i2) {
        const temp = arr[i1];
        arr[i1] = arr[i2];
        arr[i2] = temp;
    }

    // return the index of the  first element in the array `arr`, starting from `startIndex`, that satisfies the
    // testFn. If no element found, return -1
    // testFn should be a predicate function that takes 2 values to compare
    function findIndexFrom(arr, startIndex, val, testFn) {
        while (startIndex < arr.length) {
            if (testFn(arr[startIndex], val)) {
                return startIndex;
            }
            startIndex += 1;
        }
        return -1;
    }

    // 'i' is the current index of nums
    let i = 0;
    while (i < nums.length) {
        if (nums[i] === val) {
            // find first index of the first non-equal value and swap it with nums[i]
            const f = findIndexFrom(nums, i,  val, (val1, val2) => val1 !== val2 );
            if (f !== -1) {
                swap(nums, i, f);
            } else {
                // no non-equal element to val found, array only contains val elements, so we are done
                return i;
            }
        }
        i += 1;
    }
    return i;
};


let v = [2,2,2,2];
console.log(removeElement([2,2,2,2], 2)); // 0
console.log(removeElement([3,2,2,3], 2)); // 2