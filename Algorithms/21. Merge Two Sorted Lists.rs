// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(l), None) => return Some(l),
        (None, Some(r)) => return Some(r),
        (None, None) => return None,
        (Some(l), Some(r)) => {
          if l.val <= r.val {
            return Some(Box::new(ListNode{ next: Solution::merge_two_lists(l.next, Some(r)), val: l.val }));
          } else {
            return Some(Box::new(ListNode{ next: Solution::merge_two_lists(Some(l), r.next), val: r.val }));
          }
        },
      }
    }
}
