#![allow(dead_code)]

/*
206. 反转链表
简单
3.3K
相关企业
给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。


示例 1：


输入：head = [1,2,3,4,5]
输出：[5,4,3,2,1]
示例 2：


输入：head = [1,2]
输出：[2,1]
示例 3：

输入：head = []
输出：[]


提示：

链表中节点的数目范围是 [0, 5000]
-5000 <= Node.val <= 5000


进阶：链表可以选用迭代或递归方式完成反转。你能否用两种方法解决这道题？
 */

use crate::link::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut pre: Option<Box<ListNode>> = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            let next = node.next.take();
            node.next = pre;
            pre = Some(node);
            cur = next;
        }
        pre
    }
}