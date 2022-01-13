/*
Lily has a chocolate bar that she wants to share with Ron for his birthday. Each of the squares has an integer on it.
She decides to share a contiguous segment of the bar selected such that the length of the segment matches Ron's birth
month and the sum of the integers on the squares is equal to his birth day. You must determine how many ways she can
divide the chocolate.

Consider the chocolate bar as an array of squares, s = [2,2,1,3,2]
She wants to find segments summing to Ron's birth day, d = 4 with a length equalling his birth month, m = 2
i.e. Lily wants to give Ron 2 contiguous squares that sum to 4
In this case, there are two segments meeting her criteria: [2,2] and [1,3]
 */

function birthday(s, d, m) {
    const sumArr = (acc, n) => acc + n;
    let idx = 0;
    let segCount = 0;
    while (idx <= s.length - m) {
        let slice = s.slice(idx, idx + m);
        if (slice.length === m) {
            const sum = slice.reduce( sumArr, 0 );
            if (sum === d) segCount++
        }
        idx++
    }
    return segCount
}

function main() {
    const arr = [2,2,1,3,2];
    console.log("birthday res:", birthday(arr,4,2));
}