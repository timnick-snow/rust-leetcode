#![allow(dead_code)]
/*
107. 二叉树的层序遍历 II
中等
710
相关企业
给你二叉树的根节点 root ，返回其节点值 自底向上的层序遍历 。 （即按从叶子节点所在层到根节点所在的层，逐层从左向右遍历）



示例 1：


输入：root = [3,9,20,null,null,15,7]
输出：[[15,7],[9,20],[3]]
示例 2：

输入：root = [1]
输出：[[1]]
示例 3：

输入：root = []
输出：[]


提示：

树中节点数目在范围 [0, 2000] 内
-1000 <= Node.val <= 1000
 */
struct Solution;
use crate::tree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }
        let mut ans = Vec::new();
        let mut deque = VecDeque::new();
        // 根结点进入队列
        deque.push_back(Rc::clone(root.as_ref().unwrap()));

        while !deque.is_empty() {
            // 下一层节点临时队列
            let mut temp = VecDeque::new();
            // 当前层节点值
            let mut level = Vec::new();
            while !deque.is_empty() {
                let node = deque.pop_front().unwrap();
                level.push(node.borrow().val);
                // 将当前节点的左右节点入队
                if node.borrow().left.is_some() {
                    temp.push_back(Rc::clone(node.borrow().left.as_ref().unwrap()));
                }
                if node.borrow().right.is_some() {
                    temp.push_back(Rc::clone(node.borrow().right.as_ref().unwrap()));
                }
            }
            ans.push(level);
            deque = temp;
        }
        ans.reverse();
        ans
    }
}