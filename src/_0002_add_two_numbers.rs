struct Solution;
use crate::util::*;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1: &ListLink = &l1;
        let mut p2: &ListLink = &l2;
        let mut sum: ListLink = None;
        let mut p3: &mut ListLink = &mut sum;
        let mut carry: i32 = 0;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut current_sum: i32 = carry;
            if let Some(p1_) = p1 {
                current_sum += p1_.val;
                p1 = &p1_.next;
            }

            if let Some(p2_) = p2 {
                current_sum += p2_.val;
                p2 = &p2_.next;
            }
            carry = current_sum / 10;
            *p3 = ListNode::node(current_sum % 10, None);
            p3 = &mut p3.as_mut().unwrap().next;
        }
        sum
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::add_two_numbers(ListNode::list(vec![2, 4, 3]), ListNode::list(vec![5, 6, 4])),
        ListNode::list(vec![7, 0, 8])
    );
}
