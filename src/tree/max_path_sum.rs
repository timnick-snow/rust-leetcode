#![allow(dead_code)]
/*
124. 二叉树中的最大路径和
困难
2K
相关企业
二叉树中的 路径 被定义为一条节点序列，序列中每对相邻节点之间都存在一条边。同一个节点在一条路径序列中 至多出现一次 。该路径 至少包含一个 节点，且不一定经过根节点。

路径和 是路径中各节点值的总和。

给你一个二叉树的根节点 root ，返回其 最大路径和 。



示例 1：


输入：root = [1,2,3]
输出：6
解释：最优路径是 2 -> 1 -> 3 ，路径和为 2 + 1 + 3 = 6
示例 2：


输入：root = [-10,9,20,null,null,15,7]
输出：42
解释：最优路径是 15 -> 20 -> 7 ，路径和为 15 + 20 + 7 = 42


提示：

树中节点数目范围是 [1, 3 * 10^4]
-1000 <= Node.val <= 1000
 */
struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use crate::tree::TreeNode;
/*
一个节点想要与父节点构成路径时，该节点及其子树有3中可能的情况：
1. 该节点与左孩子及其衍生路径构成一条新路径
2. 该节点与右孩子及其衍生路径构成一条新路径
3. 仅该节点构成路径

因此可以计算二叉树中的一个节点的最大贡献值，就是在以该节点为根节点的子树中寻找以该节点为起点的一条路径，
使得该路径上的节点值之和最大。

 */
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = i32::MIN;
        dfs(root, 0, &mut ans);
        ans
    }
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cur: i32, ans: &mut i32) -> i32 {
    if root.is_none() {
        return 0;
    }
    let left = root.as_ref().unwrap().borrow().left.as_ref()
        .map_or(None, |x| Some(Rc::clone(x)));
    let right = root.as_ref().unwrap().borrow().right.as_ref()
        .map_or(None, |x| Some(Rc::clone(x)));

    let left_path_sum = dfs(left, cur, ans);
    let right_path_sum = dfs(right, cur, ans);
    // max_value: left+cur  right+cur left+right+cur cur
    let value = root.as_ref().unwrap().borrow().val;
    let mut max_value = value;
    if left_path_sum > 0 {
        max_value += left_path_sum;
    }
    if right_path_sum > 0 {
        max_value += right_path_sum;
    }
    *ans = std::cmp::max(*ans, max_value);
    // join path sum: cur, cur+left, cur+right
    vec![value, value + left_path_sum, value + right_path_sum]
        .into_iter().max().unwrap()
}