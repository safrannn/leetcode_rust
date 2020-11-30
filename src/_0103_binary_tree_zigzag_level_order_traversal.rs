use crate::util::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        if root.is_none() {
            return result;
        }
        let mut order_lr: bool = true;
        let mut level_nums: Vec<i32> = vec![];
        let mut visiting: Vec<Rc<RefCell<TreeNode>>> = vec![root.unwrap().clone()];
        let mut next: Vec<Rc<RefCell<TreeNode>>> = vec![];

        while let Some(node) = visiting.pop() {
            let node = node.borrow();
            level_nums.push(node.val);
            if order_lr {
                if let Some(node_left) = &node.left {
                    next.push(node_left.clone());
                }
                if let Some(node_right) = &node.right {
                    next.push(node_right.clone());
                }
            } else {
                if let Some(node_right) = &node.right {
                    next.push(node_right.clone());
                }
                if let Some(node_left) = &node.left {
                    next.push(node_left.clone());
                }
            }

            if visiting.is_empty() {
                std::mem::swap(&mut visiting, &mut next);
                result.push(level_nums.drain(..).collect());
                order_lr = !order_lr;
            }
        }

        result
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res = vec_vec_i32![[3], [20, 9], [15, 7]];
    assert_eq!(Solution::zigzag_level_order(root), res);
}
