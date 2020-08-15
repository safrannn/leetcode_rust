struct Solution;
use crate::util::*;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            return Self::is_mirror(&node.left, &node.right);
        }
        true
    }

    fn is_mirror(
        node1: &Option<Rc<RefCell<TreeNode>>>,
        node2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (node1, node2) {
            (Some(n1), Some(n2)) => {
                if n1.borrow().val != n2.borrow().val {
                    return false;
                }
                return Self::is_mirror(&n1.borrow().left, &n2.borrow().right)
                    && Self::is_mirror(&n1.borrow().right, &n2.borrow().left);
            }
            (None, None) => true,
            _ => false,
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_symmetric(tree!(
            1,
            tree!(2, tree!(3), tree!(4)),
            tree!(2, tree!(4), tree!(3))
        )),
        true
    );
}
