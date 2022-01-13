use std::collections::HashMap;

/// # Remove-Islands (medium coding challenge)
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


// NodeIndex holds a row,col index of a matrix
type NodeIndex = (usize, usize);
// type of a Node's color field
type Color = u8;
// Matrix is a 2d, square array, with dimensions of size D
type Matrix<const D: usize> = [[Color; D]; D];
// ConnMap maps a NodeIndex to true if it is connected to a border node
type ConnMap = HashMap<NodeIndex, bool>;

// Returns a list of NodeIndices that are adjacent to the node at row: `r` and col: `c`
// A node is considered adjacent if it is up,down,left or right of the given node at (r,c)
fn neighbors<const D: usize>((r, c): NodeIndex, m: &Matrix<D>) -> Vec<NodeIndex>
{
    let mut ns: Vec<NodeIndex> = vec![];
    // do initial bounds check to make sure r,c are within bounds of the array
    //if r < 0 || r >= m.len() || c < 0 || c >= m[0].len() {
    //    return ns;
    //}

    // up node
    if r > 0 {
        ns.push((r - 1, c));
    }
    // down node
    if r < m.len() - 1 {
        ns.push((r + 1, c));
    }
    // left node
    if c > 0 {
        ns.push((r, c - 1));
    }
    // right node
    if c < m[0].len() - 1 {
        ns.push((r, c + 1))
    }
    ns
}

/// returns true if the given NodeIndex is on the border of the matrix
fn is_border_node<const D: usize>((r, c): NodeIndex, m: &Matrix<D>) -> bool {
    r == 0 || r == m.len() - 1 || c == 0 || c == m[0].len() - 1
}

/// Iterates through all nodes of the given matrix and creates a "connectionMap" that maps
/// "1" nodes on the border, as well as their connected "1" node neighbors, to true
fn build_conn_map<const D: usize>(m: &Matrix<D>) -> ConnMap {
    let mut cm: ConnMap = HashMap::new();

    for r in 0..m.len() {
        for c in 0..m[0].len() {
            // if node is a "1" AND on the border, begin a depth first search to add it
            // and all its "1 colored", connected neighbors to the connMap
            if m[r][c] == 1 && is_border_node((r, c), &m) && !cm.contains_key(&(r, c)) {
                let mut to_visit: Vec<NodeIndex> = vec![(r, c)];
                while let Some(cur_node) = to_visit.pop() {
                    // mark current node as "visited" by inserting it into the connMap
                    cm.insert(cur_node, true);
                    // get connected neighbors that have NOT been visited yet
                    let nbrs = neighbors(cur_node, &m)
                        .into_iter()
                        .filter(|ni| m[ni.0][ni.1] == 1 && !cm.contains_key(ni))
                        .collect::<Vec<NodeIndex>>();
                    // add not visited connected neighbors to to_visit
                    to_visit.extend(nbrs);
                }
            }
        }
    }
    cm
}

// /// performs a depth first search and returns all "1" nodes that are connected to the given
// /// `(r,c)` index
// fn find_unvisited_connected_neighbors(
//     (r, c): NodeIndex,
//     cm: &ConnMap,
//     &m: Matrix)  -> Vec<NodeIndex>
// {
//     // holds the connected nodes
//     let mut visited = vec![];
//     let mut to_visit: Vec<NodeIndex> = vec![(r, c)];
//     while let Some(cur_node) = to_visit.pop() {
//         // mark current node as "visited"
//         visited.push(cur_node);
//         // get connected neighbors that have NOT been visited yet
//         let nbrs = neighbors(cur_node, &m)
//             .into_iter()
//             .filter(|ni| m[ni.0][ni.1] == 1 && !visited.contains(ni))
//             .collect::<Vec<NodeIndex>>();
//         // add not visited connected neighbors to to_visit
//         to_visit.extend(nbrs);
//     }
//     visited
// }


fn main() {

    let mut m1: [[Color; 6]; 6] = [
        [1, 0, 0, 0, 0, 0],
        [0, 1, 0, 1, 1, 1],
        [0, 0, 1, 0, 1, 0],
        [1, 1, 0, 0, 1, 0],
        [1, 0, 1, 1, 0, 0],
        [1, 0, 0, 0, 0, 1],
    ];


    let cm = build_conn_map(&m1);
    for r in 0..m1.len() {
        for c in 0..m1[0].len() {
            if m1[r][c] == 1 && !cm.contains_key(&(r, c)) {
                println!("not connected r:{} c:{}", r, c);
                m1[r][c] = 0;
            }
        }
    }
    // print the resulting matrix row by row
    m1.iter().for_each(|row| println!("{:?}", row));

}