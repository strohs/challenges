// The factorial of the integer , written , is defined as:
//      n! = n * (n - 1) * (n - 2) * ... * 3 * 2 * 1
//
// Calculate and return the factorial of an integer, n, where 1 <= n <= 100
//


function extraLongFactorials(n) {
    let res = BigInt(1);
    for (let i = n; i >= 1; i--) {
        res = res * BigInt(i);
    }
    console.log(res.toString());
}

extraLongFactorials(25)