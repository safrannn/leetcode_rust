#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: ListLink,
}

#[macro_export]
macro_rules! vec_string {
    ($($tail:tt),*) => {
        vec![$($tail.to_string()),*] as Vec<String>
    };
}

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
