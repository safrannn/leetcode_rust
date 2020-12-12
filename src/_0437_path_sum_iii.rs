struct Solution;
use crate::util::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut count: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        Self::traverse(sum, &root, 0, &mut count, &mut map);
        count
    }

    fn traverse(
        target: i32,
        node: &Option<Rc<RefCell<TreeNode>>>,
        prefix: i32,
        count: &mut i32,
        map: &mut HashMap<i32, i32>,
    ) {
        if let Some(node) = node {
            let node = node.borrow();
            let prefix = prefix + node.val;
            if prefix == target {
                *count += 1;
            }
            if let Some(&v) = map.get(&(target - prefix)) {
                *count += v;
            }
            *map.entry(prefix).or_insert(0) += 1;
            Self::traverse(target, &node.left, prefix, count, map);
            Self::traverse(target, &node.right, prefix, count, map);
            *map.entry(prefix).or_insert(0) -= 1;
        }
    }
}
