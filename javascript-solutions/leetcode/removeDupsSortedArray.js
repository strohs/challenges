/// # 26. Remove duplicates from a Sorted Array
/// Given a sorted array nums, remove the duplicates in-place such that each element appears only
/// once and **returns the new length.**
/// Do not allocate extra space for another array, you must do this by modifying the input array
/// in-place with *O(1)* extra memory.
///
/// ## Example 1
/// ```
/// Given nums = [1,1,2],
/// Your function should return length = 2, with the first two elements of nums being 1 and 2
/// respectively. It doesn't matter what you leave beyond the returned length.
/// ```
///
/// ## Example 2
/// ```
/// Given nums = [0,0,1,1,1,2,2,3,3,4]
/// Your function should return length = 5, with the first five elements of nums being modified
/// to 0,1,2,3 and 4 respectively. It doesn't matter what values are set beyond the returned
/// length.
/// ```

// returns the first index in `arr` containing a value that is > `num`, starting at `startIndex`
// returns null if no such element is found
function findFirstGreater(arr, startIndex, num) {
  let i = startIndex;
  while (i < arr.length) {
    if (arr[i] > num) {
      return i;
    }
    i += 1;
  }
  return null;
}

// swap elements in the array at indices si, ti
function swap(arr, si, ti) {
  let temp = arr[si];
  arr[si] = arr[ti];
  arr[ti] = temp;
}

// bubble the element at `idx` to the end of the array by repeatedly swapping it
function bubble(arr, idx) {
  while (idx + 1 < arr.length) {
    swap(arr, idx, idx+1);
    idx += 1 ;
  }
}

// remove duplicates using element bubbling and array swapping. This is a brute force technique
// var removeDuplicates = function(nums) {
//   // current index
//   let ci = 0;
//   // next index
//   let ni = ci + 1;
//
//   if (nums.length === 0) return 0;
//
//   while (ni < nums.length) {
//
//     if (nums[ni] <= nums[ci]) {
//       // next array element may be a dup, find first array element > nums[ni]
//       let fi = findFirstGreater(nums, ni, nums[ci]);
//
//       // no greater element found, dups are removed
//       if (!fi) {
//         break;
//       } else {
//         // swap ni and fi
//         swap(nums, ni, fi);
//         // bubble the element at fi to the end of the nums array
//         bubble(nums, fi);
//       }
//     }
//     ci += 1;
//     ni += 1;
//   }
//
//   return ci + 1;
// }


var removeDuplicates = function (nums) {
  if (nums.length === 0) {
    return 0;
  }

  // count keeps a "count" of the current unique elements. It is also an index that is to the "right" of all
  // the current non-duplicate elements encountered thus far.
  let count = 1;

  // i points to the current element being examined.
  // If i is equal to i-1, keep advancing i until we hit the first non-equal element.
  // If i != i-1, then swap nums[i] with nums[count] and advance count by 1.
  for (let i = 1; i < nums.length; i++) {

    if (nums[i] !== nums[i-1]) {
      nums[count] = nums[i];
      count += 1;
    }
  }
  return count;
}








/// returns the length of nums once all duplicates have been removed. Nums will be modified in-place
/// so that the non repeating elements appear at the beginning of the array, making sure to keep
/// their sorted order
// function removeDuplicates(nums) {
//   let numsIdx = 0;
//   let startIdx = numsIdx;
//
//   while (numsIdx < nums.length && startIdx < nums.length) {
//     let peekIdx = startIdx;
//
//     while (peekIdx < nums.length && nums[peekIdx] === nums[startIdx]) {
//       peekIdx++;
//     }
//
//     startIdx = (peekIdx - startIdx > 1) ? peekIdx : startIdx + 1;
//
//     nums[numsIdx] = nums[peekIdx - 1];
//     numsIdx++;
//   }
//   return numsIdx;
// }

let nums = [0,0,1,1,3,3,4,5,6,7,7,7,7,7,8];

let l = removeDuplicates(nums);
console.log(l, nums);