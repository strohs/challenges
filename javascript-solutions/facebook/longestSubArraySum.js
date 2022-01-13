/// given an array of integers, of some length, N, find the longest sub-array of integers that sum to some
/// value, S.  If there is no such sub-array, return an empty array
/// Ex.  arr = [1,3,5,4,7,5] S=12
///     the longest subarray is [3,5,4]

function findLongestSubArraySum(nums, s) {
    let longest = [];   // holds the current longest sub-array

    for (let csi = 0; csi < nums.length; csi++) {
        // index of element being examined
        let i = csi;
        // diff is the current difference of each number subtracted from s
        let diff = s;
        while (i < nums.length && diff > 0) {
            diff -= nums[i];
            i++;
        }
        if (diff === 0) {
            // we've found a sub-array that sums to S, check if it is > longest
            if (i - csi > longest.length) {
                longest = nums.slice(csi, i);
            }
        }
    }
    return longest
}

const a1 = [1,3,5,4,7,5];
const a2 = [1,2,3,4];
const a3 = [2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1];
const a4 = [13,14,15,12,15,16,17,18];

console.log("longest sub of", a1, " S=12 is", findLongestSubArraySum(a1,12));
console.log("longest sub of", a2, " S=12 is", findLongestSubArraySum(a2,12));
console.log("longest sub of", a3, " S=12 is", findLongestSubArraySum(a3,12));
console.log("longest sub of", a4, " S=12 is", findLongestSubArraySum(a4,12));
