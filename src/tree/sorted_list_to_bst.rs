#![allow(dead_code)]
/*
109. 有序链表转换二叉搜索树
中等
851
相关企业
给定一个单链表的头节点  head ，其中的元素 按升序排序 ，将其转换为高度平衡的二叉搜索树。

本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差不超过 1。



示例 1:



输入: head = [-10,-3,0,5,9]
输出: [0,-3,9,-10,null,5]
解释: 一个可能的答案是[0，-3,9，-10,null,5]，它表示所示的高度平衡的二叉搜索树。
示例 2:

输入: head = []
输出: []


提示:

head 中的节点数在[0, 2 * 104] 范围内
-105 <= Node.val <= 105
 */
use std::cell::RefCell;
use std::rc::Rc;
use crate::link::ListNode;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head.is_none() {
            return None;
        }
        let mut nums = Vec::new();
        while let Some(node) = head {
            nums.push(node.val);
            head = node.next;
        }
        helper(&nums)
    }
}

fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 0 {
        return None;
    }
    let p = (nums.len() - 1) >> 1;
    Some(Rc::new(RefCell::new(
        TreeNode {
            val: nums[p],
            left: helper(&nums[0..p]),
            right: helper(&nums[p + 1..]),
        }
    )))
}