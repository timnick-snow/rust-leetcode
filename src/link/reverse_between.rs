#![allow(dead_code)]
/*
92. 反转链表 II
中等
1.6K
相关企业
给你单链表的头指针 head 和两个整数 left 和 right ，其中 left <= right 。请你反转从位置 left 到位置 right 的链表节点，返回 反转后的链表 。


示例 1：


输入：head = [1,2,3,4,5], left = 2, right = 4
输出：[1,4,3,2,5]
示例 2：

输入：head = [5], left = 1, right = 1
输出：[5]


提示：

链表中节点数目为 n
1 <= n <= 500
-500 <= Node.val <= 500
1 <= left <= right <= n


进阶： 你可以使用一趟扫描完成反转吗？
 */
use crate::link::ListNode;

struct Solution;


impl Solution {
    /// 使用Vec存储待反转的链表结点
    pub fn reverse_between_v1(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        if left == right {
            return head;
        }
        // 哨兵
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        // p指向left前一个结点
        let mut p = &mut dummy;
        for _ in 0..left - 1 {
            p = p.next.as_mut().unwrap();
        }
        // 注意到 left<right<=n 这里的rest必然不为None
        let mut rest = p.next.take();
        let mut vec = Vec::new();
        for _ in 0..right - left + 1 {
            let temp = rest.as_mut().unwrap().next.take();
            vec.push(rest.unwrap());
            rest = temp;
        }
        // 循环结束后 rest可能为None
        // 链接反转结点
        for node in vec.into_iter().rev() {
            p.next = Some(node);
            p = p.next.as_mut().unwrap();
        }
        // 链接剩余结点
        p.next = rest;
        dummy.next
    }
}