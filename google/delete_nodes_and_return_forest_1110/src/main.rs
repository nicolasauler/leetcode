use std::{cell::RefCell, collections::HashSet, ops::Deref, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

fn del_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut forest = Vec::new();
    let set_delete: HashSet<i32> = HashSet::from_iter(to_delete.iter().cloned());

    if let Some(sub_tree) = root.clone() {
        if set_delete.get(&sub_tree.borrow().deref().val).is_some() {
            forest.push(None);
        } else {
            forest.push(Some(sub_tree.clone()));
        }

        del_nodes(sub_tree.borrow().left.clone(), to_delete.clone());
        del_nodes(sub_tree.borrow().right.clone(), to_delete);
    }

    forest
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let node_2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node_3 = Rc::new(RefCell::new(TreeNode::new(3)));

    root.borrow_mut().left = Some(node_2.clone());
    root.borrow_mut().right = Some(node_3.clone());

    let some_root = Some(root.clone());
    let to_delete = vec![1, 2];

    let a = del_nodes(some_root, to_delete);
    println!("{:?}", a);
}
