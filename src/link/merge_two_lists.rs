#![allow(dead_code)]
/*
21. 合并两个有序链表
简单
3.3K
相关企业
将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。



示例 1：


输入：l1 = [1,2,4], l2 = [1,3,4]
输出：[1,1,2,3,4,4]
示例 2：

输入：l1 = [], l2 = []
输出：[]
示例 3：

输入：l1 = [], l2 = [0]
输出：[0]


提示：

两个链表的节点数目范围是 [0, 50]
-100 <= Node.val <= 100
l1 和 l2 均按 非递减顺序 排列
 */

use crate::link::ListNode;

struct Solution;

impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = &mut dummy;
        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                let temp = list1.as_mut().unwrap().next.take();
                cur.next = list1;
                list1 = temp;
            } else {
                let temp = list2.as_mut().unwrap().next.take();
                cur.next = list2;
                list2 = temp;
            }
            cur = cur.next.as_mut().unwrap();
        }
        if list1.is_some() {
            cur.next = list1;
        }
        if list2.is_some() {
            cur.next = list2;
        }
        dummy.next
    }
}