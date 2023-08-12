#![allow(dead_code)]

/*
83. 删除排序链表中的重复元素
简单
1K
相关企业
给定一个已排序的链表的头 head ， 删除所有重复的元素，使每个元素只出现一次 。返回 已排序的链表 。



示例 1：


输入：head = [1,1,2]
输出：[1,2]
示例 2：


输入：head = [1,1,2,3,3]
输出：[1,2,3]


提示：

链表中节点数目在范围 [0, 300] 内
-100 <= Node.val <= 100
题目数据保证链表已经按升序 排列
 */
use crate::link::ListNode;

struct Solution;


impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut dummy = Box::new(ListNode::new(0));
        let mut p = dummy.as_mut();
        while let Some(mut node) = head {
            head = node.next.take();
            if pre.is_none() || node.val != pre.unwrap() {
                pre = Some(node.val);
                // 添加到新链表
                p.next = Some(node);
                p = p.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}


#[cfg(test)]
mod test {
    use crate::link::{build_list_node, node_to_vec};
    use crate::link::delete_duplicates2::Solution;

    #[test]
    pub fn t1() {
        let head = build_list_node(vec![1, 1, 2, 3, 3]);
        let ans = Solution::delete_duplicates(head);
        println!("{:?}", node_to_vec(&ans));
    }
}