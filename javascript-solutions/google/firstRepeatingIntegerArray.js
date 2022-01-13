// given an array of length N, find a digit in the array that repeats first
// the input array will only have integers between 1 ... N inclusive
// ex. [ 1, 2, 3, 1, 4, 5 ] ==> 1
// ex. [ 6, 2, 1, 2, 5, 6 ] ==> 2  (notice that 6 repeats, BUT 2 repeats before encountering the second 6)

function firstRepeating(arr) {
    let fr = -1;
    for (let n of arr) {
        n = Math.abs(n);
        if (arr[n - 1] < 0) {
            return n;
        } else {
            arr[n - 1] = arr[n - 1] * -1;
        }
    }
    return fr;
}

console.log("FR=",firstRepeating([6,2,1,2,5,6]));
console.log("FR=",firstRepeating([1,2,3,4,5,6]));
console.log("FR=",firstRepeating([1,2,3,4,3,1]));
