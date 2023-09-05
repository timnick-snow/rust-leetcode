#![allow(dead_code)]
/*
199. 二叉树的右视图
中等
939
相关企业
给定一个二叉树的 根节点 root，想象自己站在它的右侧，按照从顶部到底部的顺序，返回从右侧所能看到的节点值。



示例 1:



输入: [1,2,3,null,5,null,4]
输出: [1,3,4]
示例 2:

输入: [1,null,3]
输出: [1,3]
示例 3:

输入: []
输出: []


提示:

二叉树的节点个数的范围是 [0,100]
-100 <= Node.val <= 100
 */
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }
        let mut deque = VecDeque::new();
        deque.push_back(root.unwrap());
        let mut ans = Vec::new();
        while !deque.is_empty() {
            let mut temp = VecDeque::new();
            loop {
                let node = deque.pop_front().unwrap();
                let left = &node.borrow().left;
                if left.is_some() {
                    temp.push_back(Rc::clone(left.as_ref().unwrap()));
                }
                let right = &node.borrow().right;
                if right.is_some() {
                    temp.push_back(Rc::clone(right.as_ref().unwrap()));
                }
                if deque.is_empty() {
                    ans.push(node.borrow().val);
                    break;
                }
            }
            deque = temp;
        }
        ans
    }
}