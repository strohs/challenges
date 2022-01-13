/// # 34. Find First and Last Position of Element in Sorted Array
/// Given an array of integers `nums` sorted in ascending order, find the starting and ending
/// position of a given `target` value.
/// If target is not found in the array, return `[-1, -1]`.
/// You must write an algorithm with `O(log n)` runtime complexity.


var searchRange = function(nums, target) {

  function findRightmostIndex(arr, startIndex) {
    const target = arr[startIndex];
    let start = startIndex;
    let end = arr.length;

    while (start < end) {
      if (arr[end-1] === target) {
        return end - 1;
      }
      const mid = Math.floor((start + end) / 2);
      if (arr[mid] === target) {
        start = mid;
      } else {
        // arr[mid] > target, target is to left of mid
        end = mid;
      }
    }
    return start;
  }

  function findLeftmostIndex(arr, endIndex) {
    const target = arr[endIndex];
    let end = endIndex;
    let start = 0;

    while (start <= end) {
      if (arr[start] === target) {
        return start;
      }
      const mid = Math.floor((start + end) / 2);
      if (arr[mid] === target) {
        end = mid;
      } else {
        // arr[mid] < target, so target is to right of mid
        start = mid + 1;
      }
    }
  }
  
  let start = 0;
  let end = nums.length;

  while (start < end) {

    // base case, we have an array of length 1
    if (nums.length === 1) {
      if (nums[0] === target) {
        return [0,0];
      } else {
        return [-1, -1];
      }
    }

    // check if either nums[first] or nums[last] is == target
    if (nums[start] === target) {
      const sidx = findLeftmostIndex(nums, start);
      const eidx = findRightmostIndex(nums, start);
      return [sidx, eidx];
    }

    if (nums[end - 1] === target) {
      const sidx = findLeftmostIndex(nums, end - 1);
      const eidx = findRightmostIndex(nums, end - 1);
      return [sidx, eidx];
    }

    // otherwise check midpoint index
    const mid = Math.floor((start + end) / 2);
    if (nums[mid] === target) {
      let sidx = findLeftmostIndex(nums, mid);
      let eidx = findRightmostIndex(nums, mid);
      return [sidx, eidx];
    }

    if (target <= nums[mid]) {
      // target is on left side
      end = mid;
    } else {
      // target is on right side
      start = mid + 1;
    }
  }

  return [-1, -1];
};

const nums =  [5,7,7,8,8,10];
let res = searchRange(nums, 8);
console.log(res);

const nums2 =  [5,7,7,8,8,10];
let res2 = searchRange(nums2, 6);
console.log(res2);