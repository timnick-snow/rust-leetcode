#![allow(dead_code)]
/*
103. 二叉树的锯齿形层序遍历
中等
795
相关企业
给你二叉树的根节点 root ，返回其节点值的 锯齿形层序遍历 。（即先从左往右，再从右往左进行下一层遍历，以此类推，层与层之间交替进行）。



示例 1：


输入：root = [3,9,20,null,null,15,7]
输出：[[3],[20,9],[15,7]]
示例 2：

输入：root = [1]
输出：[[1]]
示例 3：

输入：root = []
输出：[]


提示：

树中节点数目在范围 [0, 2000] 内
-100 <= Node.val <= 100
 */
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }
        let mut ans = Vec::new();
        let mut deque = VecDeque::new();
        deque.push_back(Rc::clone(root.as_ref().unwrap()));
        let mut flag = false;
        while !deque.is_empty() {
            let mut temp = VecDeque::new();
            let mut level = Vec::new();
            while !deque.is_empty() {
                let node = deque.pop_front().unwrap();
                level.push(node.borrow().val);
                if node.borrow().left.is_some() {
                    temp.push_back(Rc::clone(node.borrow().left.as_ref().unwrap()));
                }
                if node.borrow().right.is_some() {
                    temp.push_back(Rc::clone(node.borrow().right.as_ref().unwrap()));
                }
            }
            if flag {
                level.reverse();
            }
            ans.push(level);
            deque = temp;
            flag = !flag;
        }
        ans
    }
}