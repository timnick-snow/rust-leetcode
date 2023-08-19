#![allow(dead_code)]
/*
111. 二叉树的最小深度
简单
1.1K
相关企业
给定一个二叉树，找出其最小深度。

最小深度是从根节点到最近叶子节点的最短路径上的节点数量。

说明：叶子节点是指没有子节点的节点。



示例 1：


输入：root = [3,9,20,null,null,15,7]
输出：2
示例 2：

输入：root = [2,null,3,null,4,null,5,null,6]
输出：5


提示：

树中节点数的范围在 [0, 105] 内
-1000 <= Node.val <= 1000
 */
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

/*
最小深度是从根节点到最近叶子节点的最短路径上的节点数量。
说明：叶子节点是指没有子节点的节点。
 */
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let left = root.as_ref().unwrap().borrow().left.as_ref().map_or(None, |x| Some(Rc::clone(x)));
        let right = root.as_ref().unwrap().borrow().right.as_ref().map_or(None, |x| Some(Rc::clone(x)));
        match (left, right) {
            (None, None) => 1,
            (None, Some(n2)) => 1 + Self::min_depth(Some(n2)),
            (Some(n1), None) => 1 + Self::min_depth(Some(n1)),
            (Some(n1), Some(n2)) => {
                1 + std::cmp::min(Self::min_depth(Some(n1)), Self::min_depth(Some(n2)))
            }
        }
    }
}