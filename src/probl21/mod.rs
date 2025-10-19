#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(in_val: i32) -> Self {
        ListNode {
            val: in_val,
            next: None,
        }
    }

    fn push(&mut self, ln: ListNode) -> () {
        self.next = Some(Box::new(ln));
    }
}

pub fn solution() -> () {
    let ln1 = ListNode::new(10);

    println!("Problem #21 - {:?}", ln1);
}
