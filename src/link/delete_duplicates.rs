#![allow(dead_code)]

/*
82. 删除排序链表中的重复元素 II
中等
1.2K
相关企业
给定一个已排序的链表的头 head ， 删除原始链表中所有重复数字的节点，只留下不同的数字 。返回 已排序的链表 。



示例 1：


输入：head = [1,2,3,3,4,4,5]
输出：[1,2,5]
示例 2：


输入：head = [1,1,1,2,3]
输出：[2,3]


提示：

链表中节点数目在范围 [0, 300] 内
-100 <= Node.val <= 100
题目数据保证链表已经按升序 排列
 */
use crate::link::ListNode;

struct Solution;


impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut h = dummy.as_mut();
        // 前结点
        let mut pre: Option<Box<ListNode>> = None;
        // 重复标记
        let mut flag = false;
        // 当前结点
        let mut cur: Option<Box<ListNode>>;
        while let Some(mut node) = head {
            // 剩余结点
            head = node.as_mut().next.take();
            // 当前结点
            cur = Some(node);
            // 判断是否重复
            match (cur.as_ref(), pre.as_ref()) {
                (Some(n1), Some(n2)) => {
                    if n1.val == n2.val {
                        // 标记重复
                        flag = true;
                    } else {
                        if !flag {
                            // 将pre结点加入到新链表
                            h.as_mut().unwrap().next = pre;
                            h = h.unwrap().next.as_mut();
                        }
                        flag = false;
                    }
                }
                (_, _) => ()
            }
            pre = cur;
        }
        if !flag {
            h.as_mut().unwrap().next = pre;
        }
        dummy.unwrap().next
    }
}


#[cfg(test)]
mod test {
    use crate::link::{build_list_node, node_to_vec};
    use crate::link::delete_duplicates::Solution;

    #[test]
    pub fn t1() {
        let head = build_list_node(vec![1,1,1,2,3]);
        let ans = Solution::delete_duplicates(head);
        println!("{:?}", node_to_vec(&ans));
    }
}