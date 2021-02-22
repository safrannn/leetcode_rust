struct Solution;
use crate::util::*;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::validate_child(&root, None, None)
    }

    fn validate_child(
        root: &Option<Rc<RefCell<TreeNode>>>,
        bound_low: Option<i32>,
        bound_high: Option<i32>,
    ) -> bool {
        if root.is_none() {
            return true;
        }
        if let Some(node) = root {
            let node = node.borrow();
            let value = node.val;
            if (bound_low.is_some() && value <= bound_low.unwrap())
                || (bound_high.is_some() && value >= bound_high.unwrap())
            {
                return false;
            }
            return Self::validate_child(&node.left, bound_low, Some(value))
                && Self::validate_child(&node.right, Some(value), bound_high);
        } else {
            true
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_valid_bst(tree!(2, tree!(1), tree!(3))), true);
    assert_eq!(
        Solution::is_valid_bst(tree!(5, tree!(1), tree!(4, tree!(3), tree!(6)))),
        false
    );
}
