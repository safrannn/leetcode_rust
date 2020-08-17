struct Solution;
use crate::util::*;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth: i32 = 0;
        Self::traverse(&root, 1, &mut max_depth);
        max_depth
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, depth: i32, max_depth: &mut i32) {
        if let Some(node) = root {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                if depth > *max_depth {
                    *max_depth = depth;
                }
            }
            Self::traverse(&node.borrow().left, depth + 1, max_depth);
            Self::traverse(&node.borrow().right, depth + 1, max_depth);
        }
    }
}
#[test]
fn test() {
    assert_eq!(
        Solution::max_depth(tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)))),
        3
    );
}
