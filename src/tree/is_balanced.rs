#![allow(dead_code)]
/*
110. 平衡二叉树
简单
1.4K
相关企业
给定一个二叉树，判断它是否是高度平衡的二叉树。

本题中，一棵高度平衡二叉树定义为：

一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1 。



示例 1：


输入：root = [3,9,20,null,null,15,7]
输出：true
示例 2：


输入：root = [1,2,2,3,3,null,null,4,4]
输出：false
示例 3：

输入：root = []
输出：true


提示：

树中的节点数在范围 [0, 5000] 内
-104 <= Node.val <= 104
 */
struct Solution;

use crate::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        height(root) >= 0
    }
}

fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let left = root.as_ref().unwrap().borrow().left.as_ref().map_or(None, |x| Some(Rc::clone(x)));
    let left_height = height(left);
    let right = root.as_ref().unwrap().borrow().right.as_ref().map_or(None, |x| Some(Rc::clone(x)));
    let right_height = height(right);
    if left_height == -1 || right_height == -1 || (left_height - right_height).abs() > 1 {
        -1
    } else {
        1 + std::cmp::max(left_height, right_height)
    }
}
