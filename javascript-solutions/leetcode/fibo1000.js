/// # Euler Problem 25 - First Fibonacci number with 1000 digits
/// The Fibonacci sequence is defined by the recurrence relation:
///     `Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1`
/// Hence the first 12 terms will be:
///     F1 = 1
///     F2 = 1
///     F3 = 2
///     F4 = 3
///     F5 = 5
///     F6 = 8
///     F7 = 13
///     F8 = 21
///     F9 = 34
///     F10 = 55
///     F11 = 89
///     F12 = 144
///
/// The 12th term, F12, is the first term to contain three digits.
/// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?


// returns the index of the first fibonacci number that has a length === digitLength
function fiboIndex(digitLength) {
    let idx = 0;
    let c = 1n;
    let n = 1n;

    while (c.toString(10).length !== digitLength) {
        const next = c + n;
        c = n;
        n = next;
        idx++;
    }

    return idx;
}

const idx = fiboIndex(1000);
console.log("index =", idx + 1);