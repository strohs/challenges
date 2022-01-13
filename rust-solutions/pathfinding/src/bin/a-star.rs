use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum NodeType {
    Empty,
    Obstacle,
    Start,
    Goal
}

type Link = Option<Box<Node>>;

#[derive(Debug, Clone)]
struct Node {
    f_score: i32,       // f-score
    g_score: i32,       // g-score
    r: usize,           // row index
    c: usize,           // col index
    n_type: NodeType,
    weight: i32,
    came_from: Link
}

impl Node {
    fn empty(r: usize, c: usize) -> Node {
        Node {
            n_type: NodeType::Empty,
            r,
            c,
            weight: 1,
            f_score: 1000,
            g_score: 1000,
            came_from: None
        }
    }

    fn obs(r: usize, c: usize) -> Node {
        Node {
            n_type: NodeType::Obstacle,
            r,
            c,
            weight: 100,
            f_score: 1000,
            g_score: 1000,
            came_from: None
        }
    }
    fn new(r: usize, c: usize, ntype: NodeType, weight: i32, f_score: i32, g_score: i32) -> Node {
        Node {
            n_type: ntype,
            r,
            c,
            weight,
            f_score,
            g_score,
            came_from: None
        }
    }

    /// map this nodes came_from path into a tuple of row,col indices
    fn to_indices(&self) -> Vec<(usize, usize)> {
        let mut nodes = vec![];
        let mut next_node = &self.came_from;
        while let Some(node) = next_node {
            nodes.push((node.r, node.c));
            next_node = &node.came_from;
        }
        nodes
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.c == other.c
    }
}
impl Eq for Node {}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.n_type {
            NodeType::Empty => write!(f, "_"),
            NodeType::Obstacle => write!(f, "\u{25A0}"),    // black square
            NodeType::Start => write!(f, "S"),
            NodeType::Goal => write!(f, "G")
        }
    }
}

/// Grid holds the 2D "grid" as well as the position of the starting node, and goal node.
/// cur_path is a linked list of Node(s), representing the current lowest cost path back to the
/// start node.
struct Grid {
    grid: Vec<Vec<Node>>,
    start_idx: (usize, usize),
    goal_idx: (usize, usize),
    cur_path: Vec<Node>
}

impl Grid {
    fn new5x5() -> Grid {
        let grid = vec![
            vec![Node::new(0,0,NodeType::Start, 1,0,0), Node::empty(0,1), Node::empty(0,2), Node::obs(0,3), Node::empty(0,4)],
            vec![Node::empty(1,0), Node::empty(1,1), Node::empty(1,2), Node::empty(1,3), Node::empty(1,4)],
            vec![Node::empty(2,0), Node::obs(2,1), Node::obs(2,2), Node::obs(2,3), Node::empty(2,4)],
            vec![Node::empty(3,0), Node::obs(3,1), Node::obs(3,2), Node::obs(3,3), Node::empty(3,4)],
            vec![Node::obs(4,0), Node::empty(4,1), Node::empty(4,2), Node::empty(4,3), Node::new(4,4, NodeType::Goal, 1,1000,1000)],
        ];
        Grid {
            grid,
            start_idx: (0,0),
            goal_idx: (4,4),
            cur_path: Vec::new()
        }
    }

    fn get_start(&self) -> Node {
        let (r,c) = self.start_idx;
        self.grid[r][c].to_owned()
    }

    fn get_goal(&self) -> Node {
        let (r,c) = self.goal_idx;
        self.grid[r][c].to_owned()
    }

    /// return (up to) four nodes 'around' the index at r,c
    fn nodes_adjacent_to(&self, r: usize, c: usize) -> Vec<Node> {
        let mut ns = vec![];
        if r < self.grid.len() && c < self.grid[0].len() {
            if r > 0 {
                ns.push(self.grid[r-1][c].to_owned());
            }
            if r < self.grid.len() - 1 {
                ns.push(self.grid[r+1][c].to_owned());
            }
            if c > 0 {
                ns.push(self.grid[r][c-1].to_owned());
            }
            if c < self.grid[0].len() - 1 {
                ns.push(self.grid[r][c+1].to_owned());
            }
        }
        ns
    }

    /// euclidean distance between two nodes on a grid. Uses their row/col index as x/y position
    fn distance(n1: &Node, n2: &Node) -> i32 {
        let sum: f32 = (n1.r as f32 - n2.r as f32).abs().powi(2) + (n1.c as f32 - n2.c as f32).abs().powi(2);
        sum.sqrt() as i32
    }

    fn print_grid(&self) {
        for row in &self.grid {
            for node in row {
                if self.cur_path.contains(node) && node.n_type == NodeType::Empty {
                    print!(" *");
                } else {
                    print!(" {}", node);
                }
            }
            println!();
        }
    }

    fn print_with_path(&self, cur_node: &Node) {
        let path_idxs = cur_node.to_indices();
        for row in &self.grid {
            for node in row {
                if path_idxs.contains(&(node.r, node.c)) {
                    print!(" *");
                } else {
                    print!(" {}", node);
                }
            }
            println!();
        }
    }

    /// find the lowest cost path from start_node to goal_node using A* pathfinding
    /// returns `Some(Node)` if a path was found, where `Node.came_from` will contain links
    /// to previous Node(s) in the lowest cost path.
    /// If no path to goal was found, `None` is returned
    fn a_star(&mut self) -> Option<Node> {
        // nodes's to be evaluated, sorted by a Node's lowest f-score
        let mut open_set = Vec::new();

        // nodes that have been visited
        let mut closed_set: Vec<Node> = vec![];

        open_set.push(self.get_start());

        loop {
            if !open_set.is_empty() {
                // sort open_set by f-score, so that lower f-scores are at the end
                open_set.sort_unstable_by(|n1,n2| n2.f_score.cmp(&n1.f_score));

                if let Some(cur_node) = open_set.pop() {
                    // if goal node found, we are done
                    if cur_node == self.get_goal() {
                        return Some(cur_node);
                    }
                    closed_set.push(cur_node.clone());

                    // get lowest scoring "neighbors" of the cur_node
                    let lowest_nodes = self.nodes_adjacent_to(cur_node.r, cur_node.c)
                        .into_iter()
                        // only return neighbors that are not in the closed_set
                        .filter(|n| !closed_set.contains(n))
                        // filter neighbor nodes that have a g_score lower than current path's g_score
                        .filter(|n| {
                            // g_score is the sum of all node weights from start_node to the
                            // neighbor, going through cur_node
                            let g_score = cur_node.g_score + n.weight;
                            g_score < n.g_score
                        })
                        // update the g_scores and f_scores of these nodes
                        .map(|mut n| {
                            n.came_from = Some(Box::new(cur_node.clone()));
                            n.g_score = cur_node.g_score + n.weight;
                            n.f_score = n.g_score + Grid::distance(&n, &self.get_goal());
                            n
                        })
                        .collect::<Vec<Node>>();

                    // if any lowest scoring node is not in the open_set, add it
                    for node in &lowest_nodes {
                        if !open_set.contains(&node) {
                            open_set.push(node.clone());
                        }
                    }
                }
            } else {
                // no more nodes left to explore
                return None;
            }
        }
    }
}

fn main() {
    let mut g = Grid::new5x5();
    g.print_grid();
    let path = g.a_star();
    println!("---------------------------------");
    if let Some(n) = path {
        println!("path found {:?}", &n.to_indices() );
        g.print_with_path(&n);
    } else {
        println!("NO path to Goal node was found");
    }
}