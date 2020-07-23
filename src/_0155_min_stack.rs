use std::cmp::min;
struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack { stack: vec![] }
    }
    fn push(&mut self, x: i32) {
        if self.stack.is_empty() {
            self.stack.push((x, x));
        } else {
            self.stack.push((x, min(self.stack.last().unwrap().1, x)));
        }
    }
    fn pop(&mut self) {
        self.stack.pop();
    }
    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }
    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

#[test]
fn test() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3);
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);
    assert_eq!(min_stack.get_min(), -2);
}
