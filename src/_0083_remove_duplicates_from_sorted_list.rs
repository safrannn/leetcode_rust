struct Solution;
use crate::util::*;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = head.as_mut();
        while let Some(n) = p {
            while let Some(m) = n.next.as_mut() {
                if n.val == m.val {
                    n.next = m.next.take();
                } else {
                    break;
                }
            }
            p = n.next.as_mut();
        }
        head
    }
}

#[test]
fn test() {
    let p = ListNode::list(vec![1, 1, 2, 3, 3]);
    let q = ListNode::list(vec![1, 2, 3]);
    assert_eq!(Solution::delete_duplicates(p), q);
}
