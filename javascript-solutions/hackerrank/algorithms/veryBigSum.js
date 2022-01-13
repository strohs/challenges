
function aVeryBigSum(ar) {
    return ar.reduce( (acc,n) => acc + n );
}

function main() {
    const arr = [1000000001, 1000000002, 1000000003, 1000000004, 1000000005]
    const sum = aVeryBigSum(arr)
    console.log( sum )
    return sum
}