#![allow(dead_code)]
/*
113. 路径总和 II
中等
1K
相关企业
给你二叉树的根节点 root 和一个整数目标和 targetSum ，找出所有 从根节点到叶子节点 路径总和等于给定目标和的路径。

叶子节点 是指没有子节点的节点。

示例 1：

输入：root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
输出：[[5,4,11,2],[5,8,4,5]]
示例 2：


输入：root = [1,2,3], targetSum = 5
输出：[]
示例 3：

输入：root = [1,2], targetSum = 0
输出：[]


提示：

树中节点总数在范围 [0, 5000] 内
-1000 <= Node.val <= 1000
-1000 <= targetSum <= 1000
 */
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

/*
区别在于要求出所有满足条件的路径
 */
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        back_trace(root, target_sum, &mut Vec::new(), &mut ans);
        ans
    }
}

fn back_trace(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if root.is_none() {
        return;
    }
    let left = root.as_ref().unwrap().borrow().left.as_ref()
        .map_or(None, |x| Some(Rc::clone(x)));
    let right = root.as_ref().unwrap().borrow().right.as_ref()
        .map_or(None, |x| Some(Rc::clone(x)));
    let value = root.as_ref().unwrap().borrow().val;
    match (left, right) {
        (None, None) => {
            // 当前节点为叶子节点
            if target_sum == value {
                // 找到一个路径满足条件
                path.push(value);
                ans.push(path.clone());
                path.pop();
            }
        },
        (left, right) => {
            path.push(value);
            back_trace(left, target_sum - value, path, ans);
            back_trace(right, target_sum - value, path, ans);
            path.pop();
        }
    }
}