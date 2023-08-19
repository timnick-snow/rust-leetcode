#![allow(dead_code)]
/*
108. 将有序数组转换为二叉搜索树
简单
1.4K
相关企业
给你一个整数数组 nums ，其中元素已经按 升序 排列，请你将其转换为一棵 高度平衡 二叉搜索树。

高度平衡 二叉树是一棵满足「每个节点的左右两个子树的高度差的绝对值不超过 1 」的二叉树。



示例 1：


输入：nums = [-10,-3,0,5,9]
输出：[0,-3,9,-10,null,5]
解释：[0,-10,5,null,-3,null,9] 也将被视为正确答案：

示例 2：


输入：nums = [1,3]
输出：[3,1]
解释：[1,null,3] 和 [3,1] 都是高度平衡二叉搜索树。


提示：

1 <= nums.length <= 104
-104 <= nums[i] <= 104
nums 按 严格递增 顺序排列
 */
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

/*
数组是严格递增的，中位数是根节点，左边为左子树，右边为右子树
那么，可以递归求解
 */
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        helper(&nums)
    }
}

fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 0 {
        return None;
    }
    let p = (nums.len() - 1) >> 1;
    Some(Rc::new(RefCell::new(
        TreeNode {
            val: nums[p],
            left: helper(&nums[0..p]),
            right: helper(&nums[p + 1..]),
        }
    )))
}