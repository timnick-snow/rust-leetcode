#![allow(dead_code)]
/*
145. 二叉树的后序遍历
简单
1.1K
相关企业
给你一棵二叉树的根节点 root ，返回其节点值的 后序遍历 。



示例 1：


输入：root = [1,null,2,3]
输出：[3,2,1]
示例 2：

输入：root = []
输出：[]
示例 3：

输入：root = [1]
输出：[1]


提示：

树中节点的数目在范围 [0, 100] 内
-100 <= Node.val <= 100


进阶：递归算法很简单，你可以通过迭代算法完成吗？
 */
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if root.is_none() {
                return;
            }
            let l = &root.as_ref().unwrap().borrow().left;
            let r = &root.as_ref().unwrap().borrow().right;
            dfs(l, ans);
            dfs(r, ans);
            ans.push(root.as_ref().unwrap().borrow().val);
        }
        let mut ans = Vec::new();
        dfs(&root, &mut ans);
        ans
    }
}