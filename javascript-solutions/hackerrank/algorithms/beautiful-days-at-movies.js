// Lily likes to play games with integers. She has created a new game where she determines the difference between a
// number and its reverse. For instance, given the number 12, its reverse is 21. Their difference is 9. The number
// 120 reversed is 21, and their difference is 99
// She decides to apply her game to decision making. She will look at a numbered range of days and will only go to a
// movie on a "beautiful day".
//
// Given a range of numbered days, [i..j], and a number k, determine the number of days in the range that are beautiful.
// Beautiful numbers are defined as numbers where |i - reverse(i)| is evenly divisible by k
//
// If a day's value is a beautiful number, it is a beautiful day. Print the number of beautiful days in the range.


const reverseDigits = (n) => {
    let revNum = 0;
    while (n > 0) {
        revNum = revNum * 10 + n % 10;
        n = Math.floor(n / 10);
    }
    return revNum;
};

/**
 *
 * @param i starting day number
 * @param j ending day number
 * @param k the divisor
 */
function beautifulDays(i, j, k) {
    function reverse(n) {
        return parseInt( n.toString().split('').reverse().join('') );
    }
    let beautifulDays = 0;
    for (let c = i; c <= j; c++) {
        if ( Math.abs( c - reverse(c) ) % k === 0) {
            beautifulDays++;
        }
    }
    return beautifulDays;
}

console.log( beautifulDays(20, 23, 6)); // 2

const num = 4562;
console.log( reverseDigits(num));
console.log('num =', num);