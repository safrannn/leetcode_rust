struct Solution;
use crate::util::*;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result: i32 = 0;
        Self::traverse(&root, &mut result);
        result
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut i32) {
        if let Some(node) = root {
            if let Some(node_left) = &node.borrow().left {
                if node_left.borrow().left.is_none() && node_left.borrow().right.is_none() {
                    *result += node_left.borrow().val;
                } else {
                    Self::traverse(&node.borrow().left, result);
                }
            }
            Self::traverse(&node.borrow().right, result);
        }
    }
}
#[test]
fn test() {
    assert_eq!(
        Solution::sum_of_left_leaves(tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)))),
        24
    );
}
