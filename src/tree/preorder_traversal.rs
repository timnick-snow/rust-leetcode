#![allow(dead_code)]
/*
144. 二叉树的前序遍历
简单
1.1K
相关企业
给你二叉树的根节点 root ，返回它节点值的 前序 遍历。



示例 1：


输入：root = [1,null,2,3]
输出：[1,2,3]
示例 2：

输入：root = []
输出：[]
示例 3：

输入：root = [1]
输出：[1]
示例 4：


输入：root = [1,2]
输出：[1,2]
示例 5：


输入：root = [1,null,2]
输出：[1,2]


提示：

树中节点数目在范围 [0, 100] 内
-100 <= Node.val <= 100


进阶：递归算法很简单，你可以通过迭代算法完成吗？
 */
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if root.is_none() {
                return;
            }
            ans.push(root.as_ref().unwrap().borrow().val);
            let l = &root.as_ref().unwrap().borrow().left;
            let r = &root.as_ref().unwrap().borrow().right;
            dfs(l, ans);
            dfs(r, ans);
        }
        let mut ans = Vec::new();
        dfs(&root, &mut ans);
        ans
    }

    pub fn preorder_traversal_by_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = Vec::new();
        let mut stack = Vec::new();
        stack.push(root.unwrap());
        while let Some(node) = stack.pop() {
            // 访问该节点
            ans.push(node.as_ref().borrow().val);
            // 先加入右节点 再加入左节点
            if let Some(right) = node.as_ref().borrow().right.as_ref() {
                stack.push(Rc::clone(right));
            }
            if let Some(left) = node.as_ref().borrow().left.as_ref() {
                stack.push(Rc::clone(left));
            }
        }
        ans
    }
}