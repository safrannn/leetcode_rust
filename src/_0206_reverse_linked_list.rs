struct Solution;
use crate::util::*;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = head;
        let mut prev = None;
        while let Some(mut node) = p {
            p = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::reverse_list(ListNode::list(vec![1, 2, 3, 4, 5])),
        ListNode::list(vec![5, 4, 3, 2, 1])
    );
}
