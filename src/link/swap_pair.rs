#![allow(dead_code)]

/*
24. 两两交换链表中的节点
给你一个链表，两两交换其中相邻的节点，并返回交换后链表的头节点。你必须在不修改节点内部的值的情况下完成本题（即，只能进行节点交换）。



示例 1：


输入：head = [1,2,3,4]
输出：[2,1,4,3]
示例 2：

输入：head = []
输出：[]
示例 3：

输入：head = [1]
输出：[1]


提示：

链表中节点的数目在范围 [0, 100] 内
0 <= Node.val <= 100
*/
use crate::link::ListNode;

struct Solution;

impl Solution {
    /*
      1 => 2 => 3 => 4
      2 => 1 => 4 => 3
     */
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut cur = &mut dummy;
        while head.is_some() {
            // 第二个节点及起后续节点
            let mut second_link = head.as_mut().unwrap().next.take();
            // 第一个节点
            let first = head;
            if second_link.is_none() {
                cur.as_mut().unwrap().next = first;
                break;
            }
            // 剩余链表
            head = second_link.as_mut().unwrap().next.take();
            // 反转连接 first second
            second_link.as_mut().unwrap().next = first;
            // 将反转后的两个节点连接到头节点
            cur.as_mut().unwrap().next = second_link;
            // change cur
            cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        dummy.unwrap().next
    }
}



