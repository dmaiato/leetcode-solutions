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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut cursor = head.as_mut();

        while let Some(node) = cursor {
            if let Some(next) = node.next.as_mut() {
                if node.val == next.val {
                    node.next = next.next.take();
                    cursor = Some(node); 
                } else {
                    cursor = node.next.as_mut();
                }
            } else {
                break;
            }
        }
        head
    }
}
