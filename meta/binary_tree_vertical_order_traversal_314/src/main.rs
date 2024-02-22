mod solved_with_vec;

use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    rc::Rc,
};

use solved_with_vec::test_solution_array;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

struct NodeCol {
    node: Option<Rc<RefCell<TreeNode>>>,
    col: i32,
}

/// By using BFS, we already meet a constraint of top to bottom
/// since we start with the highest nodes
fn vertical_order_hashmap(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut verticals: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    let mut queue: VecDeque<NodeCol> = VecDeque::new();
    queue.push_back(NodeCol { node: root, col: 0 });

    // while there are nodes to be processed
    while let Some(nodecol) = queue.pop_front() {
        if nodecol.node.is_none() {
            continue;
        }
        let node = nodecol.node.as_ref().unwrap();
        let col = nodecol.col;
        let val = node.borrow().val;

        verticals
            .entry(nodecol.col)
            .and_modify(|arr| arr.push(val))
            .or_insert(vec![val]);

        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();

        queue.push_back(NodeCol {
            node: left,
            col: col - 1,
        });
        queue.push_back(NodeCol {
            node: right,
            col: col + 1,
        });
    }

    return verticals.into_values().collect();
}

fn main() {
    test_solution_array();
    // root = [3,9,20,null,null,15,7]
    let mut root = TreeNode::new(3);
    let left = TreeNode::new(9);
    let right = TreeNode::new(20);
    let right_left = TreeNode::new(15);
    let right_right = TreeNode::new(7);

    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    root.right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(right_left)));
    root.right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(right_right)));

    let result = vertical_order_hashmap(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", result);
}
