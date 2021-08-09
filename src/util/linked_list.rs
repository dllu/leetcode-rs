#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn from_slice(slice: &[i32]) -> Option<Box<Self>> {
        let mut cur = None;
        for &val in slice.iter().rev() {
            let node = Self { val, next: cur };
            cur = Some(Box::new(node));
        }
        cur
    }
}
