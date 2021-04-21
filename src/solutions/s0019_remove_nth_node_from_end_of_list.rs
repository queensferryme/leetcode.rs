use super::utility::list::ListNode;
use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_nth_from_end(list: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        head.next = list;

        let mut current = &mut head;
        let mut size = 0;
        while current.next.is_some() {
            size += 1;
            current = current.next.as_deref_mut().unwrap();
        }

        let mut current = &mut head;
        let mut n = size - n;
        while n > 0 {
            n -= 1;
            current = current.next.as_deref_mut().unwrap();
        }

        current.next = current.next.clone().unwrap().next;

        return head.next;
    }
}
