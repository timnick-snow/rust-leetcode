#![allow(dead_code)]
/*
86. 分隔链表
中等
742
相关企业
给你一个链表的头节点 head 和一个特定值 x ，请你对链表进行分隔，使得所有 小于 x 的节点都出现在 大于或等于 x 的节点之前。

你应当 保留 两个分区中每个节点的初始相对位置。



示例 1：


输入：head = [1,4,3,2,5,2], x = 3
输出：[1,2,2,4,3,5]
示例 2：

输入：head = [2,1], x = 2
输出：[1,2]


提示：

链表中节点的数目在范围 [0, 200] 内
-100 <= Node.val <= 100
-200 <= x <= 200
 */
struct Solution;

use crate::link::ListNode;

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut lesser = Vec::new();
        let mut greater = Vec::new();
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                lesser.push(node);
            } else {
                greater.push(node);
            }
        }
        // 重新连接
        let mut dummy = Box::new(ListNode::new(0));
        let mut head = dummy.as_mut();
        for node in lesser.into_iter() {
            head.next = Some(node);
            head = head.next.as_mut().unwrap();
        }
        for node in greater.into_iter() {
            head.next = Some(node);
            head = head.next.as_mut().unwrap();
        }
        dummy.next
    }
}