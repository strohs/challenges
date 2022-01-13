/*
We define a magic square to be an N x N matrix of distinct positive integers from 1 to n^2 where the sum of any row,
column, or diagonal of length is always equal to the same number: the magic constant.

You will be given a 3x3 matrix of integers in the inclusive range [1..9] . We can convert any digit a to any other
digit b in the range [1,9] at cost of |a - b|. Given S, convert it into a magic square at MINIMAL COST.
Print this cost on a new line.
NOTE: the resulting magic square must contain distinct integers in the inclusive range [1..9]
p.s.: the magic constant M = n * (n^2 + 1)/ 2
p.p.s: Excluding rotations and reflections, there is exactly one 3×3 magic square

For Example: given the following matrix s:
5 3 4
1 5 8
6 4 2

can be converted to the magic square:
8 3 4
1 5 9
6 7 2

this took three replacements at a cost of |5 - 8| + |8 - 9| + |4 - 7| = 7
 */

function build3x3Transform() {
    const ms = [[8,1,6],[3,5,7],[4,9,2]];
    const m1 = [[6,1,8],[7,5,3],[2,9,4]];
    const m2 = [[2,7,6],[9,5,1],[4,3,8]];
    const m3 = [[4,3,8],[9,5,1],[2,7,6]];
    const m4 = [[2,9,4],[7,5,3],[6,1,8]];
    const m5 = [[4,9,2],[3,5,7],[8,1,6]];
    const m6 = [[6,7,2],[1,5,9],[8,3,4]];
    const m7 = [[8,3,4],[1,5,9],[6,7,2]];
    return [ms,m1,m2,m3,m4,m5,m6,m7];
}

function formingMagicSquare(ms) {
    function computeCost(m1, m2) {
        let cost = 0;
        m1.forEach( (row,ridx) => {
            row.forEach( (n, cidx) => {
                cost += Math.abs( n - m2[ridx][cidx] );
            })
        })
        return cost
    }
    const transforms = build3x3Transform();
    const costs = transforms.map( t => computeCost(ms,t));
    console.log(costs);
    return costs.sort()[0];
}