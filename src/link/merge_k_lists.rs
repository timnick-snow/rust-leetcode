#![allow(dead_code)]

/*
23. 合并 K 个升序链表
给你一个链表数组，每个链表都已经按升序排列。

请你将所有链表合并到一个升序链表中，返回合并后的链表。



示例 1：

输入：lists = [[1,4,5],[1,3,4],[2,6]]
输出：[1,1,2,3,4,4,5,6]
解释：链表数组如下：
[
1->4->5,
1->3->4,
2->6
]
将它们合并到一个有序链表中得到。
1->1->2->3->4->4->5->6

示例 2：

输入：lists = []
输出：[]

示例 3：

输入：lists = [[]]
输出：[]


提示：

k == lists.length
0 <= k <= 10^4
0 <= lists[i].length <= 500
-10^4 <= lists[i][j] <= 10^4
lists[i] 按 升序 排列
lists[i].length 的总和不超过 10^4
*/
struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::link::ListNode;


#[derive(Debug)]
struct ValNode(Box<ListNode>, usize);

impl PartialEq for ValNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.val == other.0.val
    }
}

impl PartialOrd for ValNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.val > other.0.val {
            Some(Ordering::Less)
        } else if self.0.val < other.0.val {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Eq for ValNode {}

impl Ord for ValNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<ValNode> = BinaryHeap::new();
        let mut lists: Vec<Option<Box<ListNode>>> = lists.into_iter()
            .enumerate()
            .map(|(i, x)| {
                match x {
                    None => None,
                    Some(mut node) => {
                        let next_node = std::mem::take(&mut node.next);
                        heap.push(ValNode(node, i));
                        next_node
                    }
                }
            })
            .collect::<Vec<_>>();
        let mut ans: Option<Box<ListNode>> = None;
        let mut cur = &mut ans;
        while !heap.is_empty() {
            // 弹出最小值
            let ValNode(node, i) = heap.pop().unwrap();
            // 连接到结果链表
            match cur {
                None => *cur = Some(node),
                Some(pre) => {
                    (*pre).next = Some(node);
                    cur = &mut cur.as_mut().unwrap().next;
                }
            }
            // ==从原链表中的再次添加新节点==
            // 1. 取出对应链表
            let link = std::mem::take(&mut lists[i]);
            match link {
                None => (),
                Some(mut next_candidate) => {
                    // 2. 解开头节点和下一个节点的连接，并获取下一个节点
                    let next_link = std::mem::take(&mut next_candidate.next);
                    // 3. 将当前头节点加入到二叉堆中
                    heap.push(ValNode(next_candidate,i));
                    // 4. 将下一个节点重新写入到数组对应位置
                    lists[i] = next_link;
                }
            }
        }
        ans
    }
}