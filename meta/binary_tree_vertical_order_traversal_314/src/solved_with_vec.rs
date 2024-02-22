use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut output: VecDeque<Vec<i32>> = VecDeque::new();
    let mut smallest_col = 0;

    // What if I BFS, keeping track of vert in each state?
    // that is, for root, vert = 0
    // now, for each children, left += -1 and right += 1
    // so, left = -1, right = 1
    // okay, now, same happens for those children

    // BFS vs DFS? whatever, lets try BFS

    let mut queue: VecDeque<NodeCol> = VecDeque::new();
    queue.push_front(NodeCol { node: root, col: 0 });

    while let Some(nodecol) = queue.pop_front() {
        if let None = nodecol.node {
            continue;
        }
        let node = nodecol.node.as_ref().unwrap();
        let col = nodecol.col;

        queue.push_back(NodeCol {
            node: node.borrow().left.clone(),
            col: col - 1,
        });
        queue.push_back(NodeCol {
            node: node.borrow().right.clone(),
            col: col + 1,
        });

        if col < smallest_col {
            smallest_col = col;
            output.push_front(vec![node.borrow().val]);
        } else {
            if (col - smallest_col) as usize >= output.len() {
                output.push_back(vec![node.borrow().val]);
            } else {
                output[(col - smallest_col) as usize].push(node.borrow().val);
            }
        }
    }

    return output.into();
}

pub fn test_solution_array() {
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

    let result = vertical_order(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", result);
}
