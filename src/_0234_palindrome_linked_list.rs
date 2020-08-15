struct Solution;
use rustgym::util::*;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut h_numbers: Vec<i32> = vec![];
        let mut h_next = &head;

        while let Some(h_node) = h_next {
            h_numbers.push(h_node.val);
            h_next = &h_node.next;
        }
        let length = h_numbers.len();
        for i in 0..length / 2 {
            if h_numbers[i] != h_numbers[length - 1 - i] {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_palindrome(list!(1, 2)), false);
    assert_eq!(Solution::is_palindrome(list!(1, 2, 2, 1)), true);
}
