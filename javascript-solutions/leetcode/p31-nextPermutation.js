/**
 * Leetcode Problem 31 - next permutation
 *
 * Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.
 * If such an arrangement is not possible, it must rearrange it as the lowest possible order (i.e., sorted in ascending order).
 * The replacement must be in place and use only constant extra memory.
 */




/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var nextPermutation = function(nums) {

  // returns the index of the first value in arr that is not greater than the element after it
  function firstNonAscIndex(arr) {
    if (arr.length < 2) return-1;

    let ci = arr.length - 1;
    let pi = ci - 1;

    do {
      if (arr[pi] < arr[ci]) {
        return pi;
      }
      pi -= 1;
      ci -= 1;
    } while (pi >= 0);
    return -1;
  }

  // find first index of an element in arr that satisfies the predFn, beginning at the end of the array, and ending
  // at startIndex (inclusive)
  function revFindIndex(arr, startIndex, predFn) {
    let i = arr.length - 1;
    while (i >= startIndex) {
      if (predFn(arr[i])) {
        return i;
      }
      i -= 1;
    }
    return -1;
  }

  // swap two elements in the given array at index1 and index2
  function swap(arr, idx1, idx2) {
    const temp = arr[idx1];
    arr[idx1] = arr[idx2];
    arr[idx2] = temp;
  }

  // sort a sub-slice of the given array in place. The sub-slice begins at startIndex to the end of the array.
  function arrSort(arr, startIndex) {
    let slice = arr.slice(startIndex);
    slice = slice.sort((a, b) => a - b);
    for (let i = 0; i < slice.length; i++) {
      arr[startIndex + i] = slice[i];
    }
  }

  // iterate through nums, in reverese order, looking for the first element that is less than the element after it.
  // if such an element is found, call it "fe", then we will use the sub-array to the right of "fe" and swap "fe" with
  // the first element that is greater than "fe", in that sub-array, being sure to iterate the sub-array in REVERSE
  // ORDER. After the swap, we sort the sub array and we are done.
  // If no "fe" was found. than the entire nums array is already in descending order, so sort the whole array and return

  // index of the found element "fe"
  const fIdx = firstNonAscIndex(nums);
  if (fIdx > -1) {
    // search for index of first element > nums[fi]
    const predFn = (num1, num2) => num2 > num1;
    const gtIdx = revFindIndex(nums, fIdx+1, predFn.bind(null, nums[fIdx]));
    if (gtIdx >= 0) {
      swap(nums, fIdx, gtIdx);
      arrSort(nums, fIdx + 1);
    }
  } else {
     arrSort(nums, 0);
  }

};

// const assert = require('assert');
let nums = [1, 2, 3];
nextPermutation(nums);
console.log(nums); // 1, 3 ,2

let nums2 = [3, 2, 1];
nextPermutation(nums2);
console.log(nums2); // 1, 2, 3


let nums3 = [2,2,0,4,3,1];
nextPermutation(nums3);
console.log(nums3);  // 2, 2, 1, 0, 3, 4

let nums4 = [0,0,4,2,1,0];
nextPermutation(nums4);
console.log(nums4);

let nums5 = [4,2,0,2,3,2,0];
nextPermutation(nums5);
console.log(nums5); // [4,2,0,3,0,2,2]

let nums6 = [11,12,0,27,3,11,21,9,0,15,26,27,17,24,0,16,4,17,14,8,15,8,2,16,10,6,6,24,16,2,18,19,6,10,17,10,21,0,11,13,7,7,2,16,24,25,2,20,12,9,20,19];
nextPermutation(nums6);
console.log(nums6);
//exp [11, 12,  0, 27,  3, 11, 21,  9,  0, 15, 26, 27, 17, 24,  0, 16,  4, 17, 14,  8, 15,  8, 2, 16, 10,  6,  6, 24, 16,  2, 18, 19,  6, 10, 17, 10, 21,  0, 11, 13,  7,  7,  2, 16, 24, 25,  2, 20, 12, 19, 9,  20]
//act [11, 12,  0, 27,  3, 11, 21,  9,  0, 15, 26, 27, 17, 24,  0, 16,  4, 17, 14,  8, 15,  8, 2, 16, 10,  6,  6, 24, 16,  2, 18, 19,  6, 10, 17, 10, 21,  0, 11, 13,  7,  7,  2, 16, 24, 25,  2, 20, 12, 19, 20,  9]
