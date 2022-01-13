/*
You are given an array of n integers, ar = [ar[0],ar[1]],... ar[n-1] ] and a positive integer, k.
Find and print the number of (i,j) pairs where i < j and  ar[i]+ar[j] is divisible by k

For example, ar = [1,2,3,4,5,6] and k = 5   Our three pairs meeting the criteria are [1,4],[2,3] and [4,6]
 */

/**
 * n: the integer length of array: ar
 * ar: an array of integers
 * k: the integer to divide the pair sum by
 *
 * @return number the number of pairs (i,j) where i < j and a[i] + a[j] is evenly divisible by
 */
function divisibleSumPairs(n, k, ar) {
    function buildPairs(ars) {
        let pairs = []
        for (let i=0; i < ars.length - 1; i++) {
            for (let j = i+1; j < ars.length; j++) {
                pairs.push( [ ars[i], ars[j] ])
            }
        }
        return pairs
    }

    const pairs = buildPairs( ar.sort() )
    return pairs.filter( pair => pair.reduce( (acc,elem) => acc + elem ) % k === 0).length
    // matchingPairs.forEach( pair => console.log(pair))
}