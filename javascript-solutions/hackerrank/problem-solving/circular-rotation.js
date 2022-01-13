// John Watson knows of an operation called a right circular rotation on an array of integers. One rotation operation
// moves the last array element to the first position and shifts all remaining elements right one. To test Sherlock's
// abilities, Watson provides Sherlock with an array of integers. Sherlock is to perform the rotation operation a
// number of times then determine the value of the element at a given position.
// For each array, perform a number of right circular rotations and return the value of the element at a given index.
// For example, a = [3,4,5], number of rotations k=2, and indices to check, m=[1,2]

/**
 * return an array of integers representing the values at the specified indices
 * @param a - array of integers to rotate
 * @param k - integer, number of times to rotate
 * @param queries - an array of integers containing the 0-based indices to report
 */
function circularArrayRotation(a, k, queries) {
    // compute the pivot index
    const pivotIndex = a.length - (k % a.length);
    let tempArr = [];
    if (pivotIndex <= 0) {
        tempArr = a;
    } else {
        tempArr = a.slice(pivotIndex);
        tempArr = tempArr.concat( a.slice(0, pivotIndex) );
    }
    const res = Array.of(queries.length);
    for (let i=0; i < queries.length; i++) {
        res[i] = tempArr[queries[i]];
    }
    return res;
}

console.log( circularArrayRotation([1,2,3], 2, [0,1,2]) ); // 2,3,1
console.log( circularArrayRotation([1,2,3,4], 2, [0,1,2,3]) ); // 3,4,1,2
console.log( circularArrayRotation([1,2], 1, [0,1]) ); // 2,1
console.log( circularArrayRotation([1], 1, [0]) );