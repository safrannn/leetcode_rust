use leetcode_prelude;

struct Solution;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<leetcode_prelude::ListNode>>) -> i32 {
        let mut result = 0;
        let mut p = &head;
        while let Some(n) = p {
            result *= 2;
            result += n.val;
            p = &n.next;
        }
        result
    }
}

#[test]
fn test() {
    let linkedlist1 = leetcode_prelude::linkedlist![1, 0, 1];
    assert_eq!(Solution::get_decimal_value(linkedlist1), 5);

    let linkedlist2 = leetcode_prelude::linkedlist![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0];
    assert_eq!(Solution::get_decimal_value(linkedlist2), 18880);
}
