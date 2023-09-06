#![allow(dead_code)]

/*
203. 移除链表元素
简单
1.3K
相关企业
给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。


示例 1：


输入：head = [1,2,6,3,4,5,6], val = 6
输出：[1,2,3,4,5]
示例 2：

输入：head = [], val = 1
输出：[]
示例 3：

输入：head = [7,7,7,7], val = 7
输出：[]


提示：

列表中的节点数目在范围 [0, 104] 内
1 <= Node.val <= 50
0 <= val <= 50
 */

use crate::link::ListNode;

struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;

        let mut p = &mut dummy;
        while p.as_ref().unwrap().next.is_some() {
            if p.as_ref().unwrap().next.as_ref().unwrap().val == val {
                // 移除下一个节点
                p.as_mut().unwrap().next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            } else {
                p = &mut p.as_mut().unwrap().next;
            }
        }

        dummy.unwrap().next
    }
}