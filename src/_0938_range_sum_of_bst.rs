struct Solution;
use crate::util::*;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        let mut sum: i32 = 0;
        Self::traverse(&root, l, r, &mut sum);
        sum
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32, sum: &mut i32) {
        if let Some(node) = root {
            let h_value: i32 = node.borrow().val;
            if h_value >= l && h_value <= r {
                *sum += h_value;
            }
            Self::traverse(&node.borrow().left, l, r, sum);
            Self::traverse(&node.borrow().right, l, r, sum);
        }
    }
}

#[test]
fn test() {
    let root = tree!(10, tree!(5, tree!(3), tree!(7)), tree!(15, None, tree!(18)));
    assert_eq!(Solution::range_sum_bst(root, 7, 15), 32);
}
