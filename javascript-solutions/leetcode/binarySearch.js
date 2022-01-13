
// find index of n in arr. return -1 if n is not found
function index_of(arr, n) {
    let l = 0;
    let r = arr.length - 1;

    while (l <= r) {
        let m = Math.floor((l + r) / 2);
        if (arr[m] < n) {
            l = m + 1;
        } else if (arr[m] > n) {
            r = m - 1;
        } else {
            return m;
        }
    }
    return -1;
}

const arr = [-5,-4, -3, 1, 3, 4, 5];
console.log("index of 3 is", index_of(arr, 3));
console.log("index of 10 is", index_of(arr, 10));