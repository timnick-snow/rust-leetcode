#![allow(dead_code)]
/*
129. 求根节点到叶节点数字之和
中等
670
相关企业
给你一个二叉树的根节点 root ，树中每个节点都存放有一个 0 到 9 之间的数字。
每条从根节点到叶节点的路径都代表一个数字：

例如，从根节点到叶节点的路径 1 -> 2 -> 3 表示数字 123 。
计算从根节点到叶节点生成的 所有数字之和 。

叶节点 是指没有子节点的节点。



示例 1：

输入：root = [1,2,3]
输出：25
解释：
从根到叶子节点路径 1->2 代表数字 12
从根到叶子节点路径 1->3 代表数字 13
因此，数字总和 = 12 + 13 = 25
示例 2：


输入：root = [4,9,0,5,1]
输出：1026
解释：
从根到叶子节点路径 4->9->5 代表数字 495
从根到叶子节点路径 4->9->1 代表数字 491
从根到叶子节点路径 4->0 代表数字 40
因此，数字总和 = 495 + 491 + 40 = 1026


提示：

树中节点的数目在范围 [1, 1000] 内
0 <= Node.val <= 9
树的深度不超过 10
 */
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        dfs(root, 0, &mut ans);
        ans
    }
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cur: i32, ans: &mut i32) {
    let left = root.as_ref().unwrap().borrow().left.as_ref()
        .map_or(None, |x| Some(Rc::clone(x)));
    let right = root.as_ref().unwrap().borrow().right.as_ref()
        .map_or(None, |x| Some(Rc::clone(x)));
    let value = root.as_ref().unwrap().borrow().val;
    match (left, right) {
        (None, None) => {
            *ans += cur * 10 + value;
        }
        (left, None) => {
            dfs(left, cur * 10 + value, ans);
        }
        (None, right) => {
            dfs(right, cur * 10 + value, ans);
        }
        (left, right) => {
            dfs(left, cur * 10 + value, ans);
            dfs(right, cur * 10 + value, ans);
        }
    }
}