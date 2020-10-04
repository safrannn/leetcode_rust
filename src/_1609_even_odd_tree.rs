use crate::util::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        Self::traverse(&root, &mut map, 0);

        println!("{:?}", map);
        for level in 0..map.len() as i32 {
            let current_l = level;
            if let Some(numbers) = map.get(&current_l) {
                if level % 2 == 0 {
                    for i in 0..numbers.len() {
                        if numbers[i] % 2 != 1 {
                            return false;
                        }
                    }
                    for i in 1..numbers.len() {
                        if numbers[i] <= numbers[i - 1] {
                            return false;
                        }
                    }
                } else if level % 2 == 1 {
                    for i in 0..numbers.len() {
                        if numbers[i] % 2 != 0 {
                            return false;
                        }
                    }

                    for i in 1..numbers.len() {
                        if numbers[i] >= numbers[i - 1] {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn traverse(
        root: &Option<Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<i32, Vec<i32>>,
        level: i32,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::traverse(&node.left, map, level + 1);
            let vals = map.entry(level).or_insert(vec![]);
            (*vals).push(node.val);
            Self::traverse(&node.right, map, level + 1);
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_even_odd_tree(tree!(
            5,
            tree!(4, tree!(3), tree!(3)),
            tree!(2, tree!(7), None)
        )),
        false
    );
}
