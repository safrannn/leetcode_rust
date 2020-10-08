struct Solution;
// use crate::util::*;
// use std::cell::RefCell;
// use std::rc::Rc;

// impl Solution {
//     pub fn max_depth(root: Option<Rc<RefCell<Node>>>) -> i32 {
//         let mut max_depth: i32 = 0;
//         Self::traverse(&root, 0, &mut max_depth);
//         return max_depth;
//     }

//     fn traverse(root: &Option<Rc<RefCell<Node>>>, current_depth: i32, max_depth: &mut i32) {
//         if let Some(node) = root {
//             let children = node.borrow().children; //children:Vec<&Option<Rc<RefCell<Node>>>>
//             for child in children {
//                 if let Some(child_node) = child {
//                     Self::traverse(&child_node.borrow(), current_depth + 1, max_depth);
//                 }
//             }
//         } else {
//             if *max_depth < current_depth {
//                 *max_depth = current_depth;
//             }
//         }
//     }
// }
