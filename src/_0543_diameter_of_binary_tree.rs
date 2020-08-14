struct Solution;
use crate::util::*;
use std::cell::RefCell;
use std::rc::Rc;

use std::cmp::max;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (leaf, sub) = Self::traverse(root.as_ref());
        max(leaf, sub)
    }

    fn traverse(root: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(r) = root {
            let (left_leaf, left_sub) = Self::traverse(r.borrow().left.as_ref());
            let (right_leaf, right_sub) = Self::traverse(r.borrow().right.as_ref());
            let max_l = max(left_leaf, right_leaf) + 1;
            let max_sub = max(max(left_sub, right_sub), left_leaf + right_leaf + 2);
            (max_l, max_sub)
        } else {
            return (-1, 0);
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::diameter_of_binary_tree(tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3))),
        3
    );
}
