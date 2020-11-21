use crate::util::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (_, result) = Self::dfs(&root);
        result
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        if let Some(root) = root {
            let root = root.borrow();
            let (left_i, left_b) = Self::dfs(&root.left);
            let (right_i, right_b) = Self::dfs(&root.right);
            if !left_b || !right_b || (left_i - right_i).abs() > 1 {
                return (-1, false);
            } else {
                return (left_i.max(right_i) + 1, true);
            }
        }
        return (0, true);
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_balanced(tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)))),
        true
    );
}
