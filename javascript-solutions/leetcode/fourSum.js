/// Given an array nums of n integers, are there elements a, b, c, d in nums such that a + b + c + d = target ?
/// Find all unique quadruplets in the array which gives the sum of `target`.
///
/// ## Note:
/// The solution set must not contain duplicate quadruplets.
///
/// ## Example
/// ```
/// Given array nums = [-1, 0, 1, 0, -2, 2], and target = 0
///
/// A solution set is:
/// [
///   [-1, 0, 0, 1],
///   [-2, -1, 1, 2],
///   [-2, 0, 0, 2]
/// ]
/// ```


// compare two Arrays of integers for equality
function equalArrays(array1, array2) {
    return array1.length === array2.length && array1.every((value, index) => value === array2[index]);
}


/**
 * return all quadruples of integers that add up to the given sum
 * @param nums - array of integers
 * @param sum - integer sum
 * @returns {[[Number, Number,Number,Number]]} - set of arrays where each array contains
 * a quadruplet of integers that add up to sum
 */
function fourSum(nums, sum) {
    const quads = [];
    for (let idx=0; idx < nums.length; idx++) {
        const target = sum - nums[idx];
        const sums = threeSum(nums.slice(idx+1), target);

        for (let trips of sums) {
            trips.push(nums[idx]);
            trips.sort();
            // only add distinct quadruplets to the final quadruplets array
            if (!quads.some(arr => equalArrays(arr, trips))) {
                quads.push(trips);
            }
        }
    }

    return quads;
}


/**
 * return all triplets of integers that add up to the given sum
 * @param nums - array of integers
 * @param sum - integer sum
 * @returns {[[Number,Number,Number]]} - set of arrays where each array contains
 * a triplet of integers that add up to sum
 */
function threeSum(nums, sum) {
    const triplets = [];
    for (let idx=0; idx < nums.length; idx++) {
        const target = sum - nums[idx];
        const sums = twoSum(nums.slice(idx+1), target);
        for (let pairs of sums) {
            pairs.push(nums[idx]);
            pairs.sort();
            // only add distinct triplets to the final triplets array
            if (!triplets.some(arr => equalArrays(arr, pairs))) {
                triplets.push(pairs);
            }
        }
    }
    return triplets;
}


/**
 * return all pairs of integers in nums that add up to sum
 * @param nums - array of integers
 * @param sum - the target sum
 * @returns {[[Number, Number]]} - an array containing arrays of integer pairs
 */
function twoSum(nums, sum) {
    const numMap = new Map();
    const pairs = [];
    for (let n of nums) {
        const target = sum - n;
        if (numMap.has(target)) {
            pairs.push([n, target]);
        }
        if (!numMap.has(n)) numMap.set(n, 1);
    }
    return pairs;
}


let input1 = [-1,0,1,0,2,-2];
const res1 = fourSum(input1, 0);
console.log(res1); // [ [-1,0,0,1], [-2,-1,-1,2], [-2,0,0,2] ]

let input2 = [1,2,1,2,1,2,1];
const res2 = fourSum(input2, 6);
console.log(res2); // [ [1,1,2,2] ]