use crate::util::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_some() {
            Self::dfs(&root, sum)
        } else {
            false
        }
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if let Some(root) = root {
            let root = root.borrow();
            match (&root.left, &root.right) {
                (None, None) => sum == root.val,
                (left, right) => {
                    if (left.is_some() && Self::dfs(left, sum - root.val))
                        || (right.is_some() && Self::dfs(right, sum - root.val))
                    {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        } else {
            sum == 0
        }
    }
}

#[test]
fn test() {
    let root = tree!(
        5,
        tree!(4, tree!(11, tree!(7), tree!(2)), None),
        tree!(8, tree!(13), tree!(4, None, tree!(1)))
    );
    assert_eq!(Solution::has_path_sum(root, 22), true);
    assert_eq!(Solution::has_path_sum(tree!(1, tree!(2), None), 1), false);
}
