#![allow(dead_code)]
/*
148. 排序链表
中等
2.1K
相关企业
给你链表的头结点 head ，请将其按 升序 排列并返回 排序后的链表 。



示例 1：


输入：head = [4,2,1,3]
输出：[1,2,3,4]
示例 2：


输入：head = [-1,5,3,4,0]
输出：[-1,0,3,4,5]
示例 3：

输入：head = []
输出：[]


提示：

链表中节点的数目在范围 [0, 5 * 104] 内
-105 <= Node.val <= 105


进阶：你可以在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序吗？
 */
use crate::link::ListNode;

struct Solution;

impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut arr = Vec::new();
        while let Some (mut n) = head{
            head = n.next.take();
            arr.push(n);
        }
        arr.sort_unstable_by_key(|x| x.val);

        let mut dummy = Box::new(ListNode::new(0));
        let mut p = &mut dummy;
        for node in arr.into_iter() {
            p.next = Some(node);
            p = p.next.as_mut().unwrap();
        }
        dummy.next
    }
}