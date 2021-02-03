use super::utility::list::ListNode;
use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut current = &mut result;
        let mut tuple = (l1, l2, 0);

        loop {
            tuple = match tuple {
                (Some(j1), Some(j2), carry) => (j1.next, j2.next, j1.val + j2.val + carry),
                (Some(j1), None, carry) => (j1.next, None, j1.val + carry),
                (None, Some(j2), carry) => (None, j2.next, j2.val + carry),
                (None, None, carry) if carry > 0 => (None, None, carry),
                _ => {
                    break;
                }
            };
            *current = Some(Box::new(ListNode::new(tuple.2 % 10)));
            current = &mut current.as_mut().unwrap().next;
            tuple.2 /= 10;
        }

        return result;
    }
}
