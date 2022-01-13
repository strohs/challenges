// find the first and last position of an element in a sorted array
// the algorithm should run in O(log n) time
// Ex.
// given array: [5, 7, 7, 8, 8, 10] target = 8
//   return: [3, 4]
//
// Ex 2.
// given array: [5, 7, 7, 8, 8, 10] target = 6
//   return : [-1, -1]


function findFirstLastPos(nums, target) {
    // find the middle index between si=start index, and ei = end index
    const midIndex = (si, ei) => (Math.round((ei - si) / 2)) + si;

    // check if target is within the array
    //if (target < nums[0] || target > nums[nums.length - 1]) {
    //    return [-1, -1];
    //}

    // the entire array is filled with target
    if (nums[0] === target && nums[nums.length - 1] === target) {
        return [0, nums.length - 1];
    }

    let si = 0;                 // start index
    let ei = nums.length - 1;   // end index
    let cmi = midIndex(si, ei); // current middle index
    let spos = -1;
    let epos = -1;

    // repeatedly search half of the array for a middle index that == target, or until starting index === ending index
    while (nums[cmi] !== target && si < ei && ei > si) {
        if (target < nums[cmi]) {
            ei = cmi - 1;
        } else {
            si = cmi + 1;
        }
        cmi = midIndex(si, ei);
    }

    if (nums[cmi] === target) {
        // iterate backwards until the beginning of the target is found
        spos = cmi;
        while ((spos - 1) >= 0 && nums[spos - 1] === target) {
            spos--;
        }
        // iterate forwards until the end of the target is found
        epos = cmi;
        while ((epos + 1) < nums.length && nums[epos + 1] === target) {
            epos++;
        }
    }
    return [spos, epos];
}

console.log("[3, 4] target 3 = ", findFirstLastPos([3, 4], 3));
console.log("[3, 4] target 6 = ", findFirstLastPos([3, 4], 6));
console.log("[5,7,7,8,8,10] target 8 = ", findFirstLastPos([5,7,7,8,8,10], 8));
console.log("[5,7,7,8,8,10] target 9 = ", findFirstLastPos([5,7,7,8,8,10], 9));
console.log("[5,7,7,8,8,9,10] target 9 = ", findFirstLastPos([5,7,7,8,8,9,10], 9));