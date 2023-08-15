#![allow(dead_code)]
/*
94. 二叉树的中序遍历
简单
1.9K
相关企业
给定一个二叉树的根节点 root ，返回 它的 中序 遍历 。



示例 1：


输入：root = [1,null,2,3]
输出：[1,3,2]
示例 2：

输入：root = []
输出：[]
示例 3：

输入：root = [1]
输出：[1]


提示：

树中节点数目在范围 [0, 100] 内
-100 <= Node.val <= 100


进阶: 递归算法很简单，你可以通过迭代算法完成吗？
 */
struct Solution;
use crate::tree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        inorder(root.as_ref(), &mut ans);
        ans
    }
}

fn inorder(root: Option<&Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }
    inorder(root.unwrap().borrow().left.as_ref(), ans);
    ans.push(root.unwrap().borrow().val);
    inorder(root.unwrap().borrow().right.as_ref(), ans);
}