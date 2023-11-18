#![allow(dead_code)]
/*
404. 左叶子之和
简单
相关标签
相关企业
给定二叉树的根节点 root ，返回所有左叶子之和。



示例 1：



输入: root = [3,9,20,null,null,15,7]
输出: 24
解释: 在这个二叉树中，有两个左叶子，分别是 9 和 15，所以返回 24
示例 2:

输入: root = [1]
输出: 0


提示:

节点数在 [1, 1000] 范围内
-1000 <= Node.val <= 1000
 */

use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        dfs(&root, false, &mut sum);
        sum
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool, sum: &mut i32) {
    if node.is_none() {
        return;
    }
    let left = &node.as_ref().unwrap().borrow().left;
    let right = &node.as_ref().unwrap().borrow().right;
    if is_left && left.is_none() && right.is_none() {
        // 找到一个左叶子
        *sum += node.as_ref().unwrap().borrow().val;
        return;
    }

    dfs(left, true, sum);
    dfs(right, false, sum);
}