// vector of string
#[macro_export]
macro_rules! vec_string {
    ($($tail:tt),*) => {
        vec![$($tail.to_string()),*] as Vec<String>
    };
}

// Linked List
// ListNode struct in leetcode
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: ListLink,
}

// implementation
pub type ListLink = Option<Box<ListNode>>;

impl ListNode {
    pub fn list(v: Vec<i32>) -> ListLink {
        let mut next: ListLink = None;
        for &val in v.iter().rev() {
            next = Some(Box::new(ListNode { val, next }))
        }
        next
    }
}

impl ListNode {
    pub fn node(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode { val, next }))
    }
}

// Binary Tree
// TreeNode structure in leetcode
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeLink,
    pub right: TreeLink,
}

// implementation
use std::cell::RefCell;
use std::rc::Rc;

pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

pub trait MakeTree {
    fn branch(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    fn leaf(val: i32) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

impl MakeTree for TreeLink {}

#[macro_export]
macro_rules! tree {
    ($e:expr) => {
        TreeLink::leaf($e)
    };
    ($e:expr, $l:expr, $r:expr) => {
        TreeLink::branch($e, $l, $r)
    };
}
