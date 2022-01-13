/// # Remove-Islands medium coding challenge
/// You are given a 2d matrix consisting of 1's and 0's.
/// 1 indicates a "black" pixel and 0 indicates a "white" pixel.
/// Your task is to remove all black colored pixels that are **NOT** connected to the border
/// of the matrix, by replacing them with 0.
/// You should return the (modified) input matrix
///
/// ## Example
/// Given the following matrix
/// ```
/// let mut m1: [[u8; 6]; 6] = [
///         [1, 0, 0, 0, 0, 0],
///         [0, 1, 0, 1, 1, 1],
///         [0, 0, 1, 0, 1, 0],
///         [1, 1, 0, 0, 1, 0],
///         [1, 0, 1, 1, 0, 0],
///         [1, 0, 0, 0, 0, 1],
///     ];
/// ```
/// the cells at (1,1) (2,2), (4,2) and (4,3) should be replaced with 0


/**
 * returns the indices of nodes that are adjacent to the node it index r,c. A node is adjacent if it
 * up. down, left or right
 * @param r {int}
 * @param c {int}
 * @param mtx {[[int]]} a 2d array
 * @returns [{r: int, c: int}] an array of {r, c} objects that contain the indices of neighbors nodes
 */
function neighbors(r, c, mtx) {
    const ns = [];
    // up node
    if (r > 0) {
        ns.push({ r: r-1, c: c });
    }
    // down node
    if (r < mtx.length - 1) {
        ns.push({ r: r+1, c: c });
    }
    // left node
    if (c > 0) {
        ns.push({ r: r, c: c-1 });
    }
    // right node
    if (c < mtx[0].length- 1) {
        ns.push({ r: r, c: c+1})
    }
    return ns;
}

/**
 * returns true if the given r,c index in mtx is on the border of the mtx
 * @param r {int} row index
 * @param c {int} col index
 * @param mtx {[[int]]} 2d array of integers
 * @returns {boolean}
 */
function isBorderNode(r, c, mtx) {
    return r === 0 || r === mtx.length - 1 || c === 0 || c === mtx[0].length - 1
}

/**
 * Iterates through all nodes of the given matrix and creates a "connectionMap" that maps
 * "1" nodes on the border, as well as their connected "1" node neighbors, to true
 * @param mtx {[[int]]}
 * @returns {Map<string, boolean>}
 */
function buildConnMap(mtx) {
    // the connection map, maps {r, c} object to boolean
    const cm = new Map();

    for (const [r, row] of mtx.entries()) {
        for (const [c, color] of row.entries()) {
            if (color === 1 && isBorderNode(r, c, mtx) && !cm.has(JSON.stringify({r: r, c: c}))) {
                let toVisit = [{r: r, c: c}];
                while (toVisit.length > 0) {
                    let curNode = toVisit.pop();
                    cm.set(JSON.stringify(curNode), true);
                    // get connected. non-visited neighbors of curNode
                    const nbrs = neighbors(curNode.r, curNode.c, mtx)
                        .filter(node => mtx[node.r][node.c] === 1 && !cm.has(JSON.stringify(node)));
                    toVisit =  toVisit.concat(nbrs);
                }
            }
        }
    }
    return cm;
}

const m1 = [
    [1, 0, 0, 0, 0, 0],
    [0, 1, 0, 1, 1, 1],
    [0, 0, 1, 0, 1, 0],
    [1, 1, 0, 0, 1, 0],
    [1, 0, 1, 1, 0, 0],
    [1, 0, 0, 0, 0, 1],
];

const cm = buildConnMap(m1);
for (let r = 0; r < m1.length; r++) {
    for (let c = 0; c < m1[0].length; c++) {
        if (m1[r][c] === 1 && !cm.has(JSON.stringify({r: r, c: c}))) {
            console.log(`${r}:${c} is not connected`);
        }
    }
}