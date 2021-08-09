use crate::util::linked_list::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;

    let mut end = false;
    let mut carry = 0;
    let mut lol = Vec::new();
    while !end {
        end = true;
        let mut val = carry;
        if let Some(node_1) = l1 {
            val += node_1.val;
            l1 = node_1.next;
            end = false;
        }
        if let Some(node_2) = l2 {
            val += node_2.val;
            l2 = node_2.next;
            end = false;
        }

        carry = val / 10;
        val %= 10;
        if !end || val > 0 {
            lol.push(val);
        }
    }

    let mut cur = None;
    for &val in lol.iter().rev() {
        let node = ListNode { val, next: cur };
        cur = Some(Box::new(node));
    }
    cur
}

#[cfg(test)]
mod tests {
    use crate::util::linked_list::ListNode;
    #[test]
    fn q00002() {
        assert_eq!(
            super::add_two_numbers(
                ListNode::from_slice(&[2, 4, 3]),
                ListNode::from_slice(&[5, 6, 4])
            ),
            ListNode::from_slice(&[7, 0, 8])
        );
    }
}
