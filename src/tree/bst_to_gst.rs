#![allow(dead_code)]

/*
1038. 从二叉搜索树到更大和树
中等
相关标签
相关企业
提示
给定一个二叉搜索树 root (BST)，请将它的每个节点的值替换成树中大于或者等于该节点值的所有节点值之和。

提醒一下， 二叉搜索树 满足下列约束条件：

节点的左子树仅包含键 小于 节点键的节点。
节点的右子树仅包含键 大于 节点键的节点。
左右子树也必须是二叉搜索树。


示例 1：



输入：[4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
输出：[30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
示例 2：

输入：root = [0,null,1]
输出：[1,null,1]


提示：

树中的节点数在 [1, 100] 范围内。
0 <= Node.val <= 100
树中的所有值均 不重复 。


注意：该题目与 538: https://leetcode-cn.com/problems/convert-bst-to-greater-tree/  相同
 */

use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

/*
递归
右 -> 中 -> 左
 */
impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        dfs(&root, 0);
        root
    }
}

fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, parent_sum: i32) -> i32 {
    if root.is_none() {
        return 0;
    }
    let value = root.as_ref().unwrap().borrow().val;
    // 右子树的和
    let right_sum = dfs(&root.as_ref().unwrap().borrow().right, parent_sum);
    // 计算当前节点的新值
    let new_value = value + right_sum + parent_sum;
    root.as_ref().unwrap().borrow_mut().val = new_value;

    // 处理左子树
    let left_sum = dfs(&root.as_ref().unwrap().borrow().left, new_value);

    left_sum + right_sum + value
}