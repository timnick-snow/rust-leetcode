#![allow(dead_code)]
/*
112. 路径总和
简单
1.2K
相关企业
给你二叉树的根节点 root 和一个表示目标和的整数 targetSum 。判断该树中是否存在 根节点到叶子节点 的路径，这条路径上所有节点值相加等于目标和 targetSum 。如果存在，返回 true ；否则，返回 false 。

叶子节点 是指没有子节点的节点。



示例 1：


输入：root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
输出：true
解释：等于目标和的根节点到叶节点路径如上图所示。
示例 2：


输入：root = [1,2,3], targetSum = 5
输出：false
解释：树中存在两条根节点到叶子节点的路径：
(1 --> 2): 和为 3
(1 --> 3): 和为 4
不存在 sum = 5 的根节点到叶子节点的路径。
示例 3：

输入：root = [], targetSum = 0
输出：false
解释：由于树是空的，所以不存在根节点到叶子节点的路径。


提示：

树中节点的数目在范围 [0, 5000] 内
-1000 <= Node.val <= 1000
-1000 <= targetSum <= 1000
 */
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        dfs(root, 0, target_sum)
    }
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cur: i32, target_sum: i32) -> bool {
    let left = root.as_ref().unwrap().borrow().left.as_ref()
        .map_or(None, |x| Some(Rc::clone(x)));
    let right = root.as_ref().unwrap().borrow().right.as_ref()
        .map_or(None, |x| Some(Rc::clone(x)));

    let value = root.as_ref().unwrap().borrow().val;
    match (left, right) {
        (None, None) => value + cur == target_sum,
        (None, right) => dfs(right, cur + value, target_sum),
        (left, None) => dfs(left, cur + value, target_sum),
        (left, right) => {
            dfs(right, cur + value, target_sum)
                || dfs(left, cur + value, target_sum)
        }
    }
}