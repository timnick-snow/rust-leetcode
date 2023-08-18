#![allow(dead_code)]
/*
104. 二叉树的最大深度
简单
1.7K
相关企业
给定一个二叉树 root ，返回其最大深度。

二叉树的 最大深度 是指从根节点到最远叶子节点的最长路径上的节点数。



示例 1：


输入：root = [3,9,20,null,null,15,7]
输出：3
示例 2：

输入：root = [1,null,2]
输出：2


提示：

树中节点的数量在 [0, 104] 区间内。
-100 <= Node.val <= 100
 */
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            0
        } else {
            let left = root.as_ref().unwrap().borrow().left.as_ref()
                .map_or(None, |x| Some(Rc::clone(x)));
            let right = root.as_ref().unwrap().borrow().right.as_ref()
                .map_or(None, |x| Some(Rc::clone(x)));
            1 + std::cmp::max(Self::max_depth(left), Self::max_depth(right))
        }
    }
}