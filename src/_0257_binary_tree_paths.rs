struct Solution;
use crate::util::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn helper(
            root: &Option<Rc<RefCell<TreeNode>>>,
            current_path: &mut String,
            paths: &mut Vec<String>,
        ) {
            if let Some(root) = root {
                let root = root.borrow();
                let current_value = format!("{}", root.val);
                current_path.push_str(&current_value);
                if root.left.is_none() && root.right.is_none() {
                    paths.push(current_path.clone());
                } else {
                    current_path.push_str("->");
                    helper(&root.left, current_path, paths);
                    helper(&root.right, current_path, paths);
                    current_path.truncate(current_path.len() - 2);
                }
                current_path.truncate(current_path.len() - current_value.len());
            }
        }
        let mut current_path = String::new();
        let mut paths = Vec::new();
        helper(&root, &mut current_path, &mut paths);
        paths
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, None, tree!(5)), tree!(3));
    let paths: Vec<String> = vec_string!["1->2->5", "1->3"];
    assert_eq!(Solution::binary_tree_paths(root), paths);
}
