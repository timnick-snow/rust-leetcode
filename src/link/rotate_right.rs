#![allow(dead_code)]
/*
61. 旋转链表
中等
958
相关企业
给你一个链表的头节点 head ，旋转链表，将链表每个节点向右移动 k 个位置。



示例 1：


输入：head = [1,2,3,4,5], k = 2
输出：[4,5,1,2,3]
示例 2：


输入：head = [0,1,2], k = 4
输出：[2,0,1]


提示：

链表中节点的数目在范围 [0, 500] 内
-100 <= Node.val <= 100
0 <= k <= 2 * 10^9
 */
use crate::link::ListNode;

struct Solution;

/*
先移动到旋转点的前一个节点 pre
a0 a1 ... pre b1 .. bk

转化成目标形态
b1 .. bk a0 a1 ... pre
 */
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 1. 计算链表长度
        let mut len = 0;
        let mut p = head.as_ref();
        while let Some(node) = p {
            len += 1;
            p = node.next.as_ref();
        }
        if len == 0 {
            return head;
        }
        // 实际位移
        let k = k % len;
        if k == 0 {
            return head;
        }
        // 定位pre结点
        let mut p = head.as_mut();
        for _ in 0..len - k - 1 {
            p = p.unwrap().next.as_mut();
        }
        let mut ans = p.unwrap().next.take();
        // 定位尾结点
        let mut p = ans.as_mut();
        // while let Some(node) = p.unwrap().next.as_mut() {
        //     p = Some(node);
        // }
        for _ in 0..k - 1 {
            p = p.unwrap().next.as_mut();
        }
        p.unwrap().next = head;
        ans
    }
}