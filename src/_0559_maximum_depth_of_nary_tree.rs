struct Solution;
use crate::util::*;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn maxDepth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth: i32 = 0;
        Self::traverse(&root, 0, &mut max_depth);
        max_depth
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, current_depth: i32, max_depth: &mut i32) {
        if let Some(node) = root {
            Self::traverse(&node.borrow().left, current_depth + 1, max_depth);
            Self::traverse(&node.borrow().right, current_depth + 1, max_depth);
        } else {
            if *max_depth < current_depth {
                *max_depth = current_depth;
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::maxDepth(tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)))),
        3
    );
}
