struct Solution;
use crate::util::*;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn merge_trees(
        mut t1: Option<Rc<RefCell<TreeNode>>>,
        mut t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&mut t1, &mut t2);
        t1
    }

    fn helper(t1: &mut Option<Rc<RefCell<TreeNode>>>, t2: &mut Option<Rc<RefCell<TreeNode>>>) {
        if t1.is_some() && t2.is_some() {
            let (mut t1_, mut t2_) = (
                t1.as_mut().unwrap().borrow_mut(),
                t2.as_mut().unwrap().borrow_mut(),
            );
            t1_.val += t2_.val;
            Self::helper(&mut t1_.left, &mut t2_.left);
            Self::helper(&mut t1_.right, &mut t2_.right)
        } else if t2.is_some() {
            *t1 = t2.take();
        }
    }
}

#[test]
fn test() {
    let t1 = tree!(1, tree!(3, tree!(5), None), tree!(2));
    let t2 = tree!(2, tree!(1, None, tree!(4)), tree!(3, None, tree!(7)));
    let res = tree!(3, tree!(4, tree!(5), tree!(4)), tree!(5, None, tree!(7)));
    assert_eq!(Solution::merge_trees(t1, t2), res);
}
