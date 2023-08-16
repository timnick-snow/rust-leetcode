#![allow(dead_code)]
/*
98. 验证二叉搜索树
中等
2.1K
相关企业
给你一个二叉树的根节点 root ，判断其是否是一个有效的二叉搜索树。

有效 二叉搜索树定义如下：

节点的左子树只包含 小于 当前节点的数。
节点的右子树只包含 大于 当前节点的数。
所有左子树和右子树自身必须也是二叉搜索树。


示例 1：


输入：root = [2,1,3]
输出：true
示例 2：


输入：root = [5,1,4,null,null,3,6]
输出：false
解释：根节点的值是 5 ，但是右子节点的值是 4 。


提示：

树中节点数目范围在[1, 104] 内
-231 <= Node.val <= 231 - 1
 */
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        valid(root.as_ref(), None, None)
    }
}

fn valid(root: Option<&Rc<RefCell<TreeNode>>>, lower: Option<i32>, upper: Option<i32>) -> bool {
    if root.is_none() {
        return true;
    }
    // 验证当前结点的上下界
    let value = root.as_ref().unwrap().borrow().val;
    if lower.is_some() && value <= lower.unwrap() {
        return false;
    }
    if upper.is_some() && value >= upper.unwrap() {
        return false;
    }
    // 验证左子树
    let left_valid = valid(root.as_ref().unwrap().borrow().left.as_ref(), lower, Some(value));
    if !left_valid {
        return false;
    }
    // 验证右子树
    let right_valid = valid(root.as_ref().unwrap().borrow().right.as_ref(), Some(value), upper);
    if !right_valid {
        return false;
    }
    // 左右子树都满足
    true
}