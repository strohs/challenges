
// combine two sorted arrays into a single array
function combine(l,r) {
    const marr = [];
    let i=0, j=0;
    while (i < l.length && j < r.length) {
        if (l[i] <= r[j]) {
            marr.push(l[i]);
            i++;
        } else {
            marr.push(r[j]);
            j++;
        }
    }
    while (i < l.length) marr.push(l[i++]);
    while (j < r.length) marr.push(r[j++]);
    return marr;
}

function merge_sort(arr) {
    if (arr.length <= 1) {
        return arr;
    }
    const mp = arr.length / 2;
    let l = arr.slice(0, mp);
    let r = arr.slice(mp, arr.length);

    // recursively sort and merge both sublists,
    l = merge_sort(l);
    r = merge_sort(r);
    return combine(l, r);
}

const a1 = [7,2,7,2,8,3,9,6,4,1,0,5];
const sorted = merge_sort(a1);
console.log(sorted);
