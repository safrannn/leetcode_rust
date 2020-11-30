use crate::util::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let answer = Self::traverse(root.as_ref());
        return answer.0.max(answer.1);
    }
    fn traverse(root: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(root) = root {
            let root = root.borrow();
            let value = root.val;
            let left = Self::traverse(root.left.as_ref());
            let right = Self::traverse(root.right.as_ref());
            (
                value + left.1 + right.1,
                left.0.max(left.1) + right.0.max(right.1),
            )
        } else {
            (0, 0)
        }
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(2, None, tree!(3)), tree!(3, None, tree!(1)));
    let res = 7;
    assert_eq!(Solution::rob(root), res);
    let root = tree!(3, tree!(4, tree!(1), tree!(3)), tree!(5, None, tree!(1)));
    let res = 9;
    assert_eq!(Solution::rob(root), res);
}
