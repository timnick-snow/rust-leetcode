#![allow(dead_code)]

/*
437. 路径总和 III
中等
相关标签
相关企业
给定一个二叉树的根节点 root ，和一个整数 targetSum ，求该二叉树里节点值之和等于 targetSum 的 路径 的数目。

路径 不需要从根节点开始，也不需要在叶子节点结束，但是路径方向必须是向下的（只能从父节点到子节点）。



示例 1：



输入：root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
输出：3
解释：和等于 8 的路径有 3 条，如图所示。
示例 2：

输入：root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
输出：3


提示:

二叉树的节点个数的范围是 [0,1000]
-109 <= Node.val <= 109
-1000 <= targetSum <= 1000
 */
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut map = HashMap::new();
        map.insert(target_sum as i64, 1);
        help(root.as_ref().unwrap().clone(), map, target_sum as i64)
    }
}

fn help(root: Rc<RefCell<TreeNode>>, target: HashMap<i64, i32>, target_sum: i64) -> i32 {
    let val = root.borrow().val as i64;
    let a = *target.get(&val).unwrap_or(&0);

    let left = root.borrow().left.as_ref()
        .map_or(None, |x| Some(x.clone()));
    let right = root.borrow().right.as_ref()
        .map_or(None, |x| Some(x.clone()));

    let b = match left {
        None => 0,
        Some(n) => {
            help(n, choose(&target, val, target_sum), target_sum)
        }
    };
    let c = match right {
        None => 0,
        Some(n) => {
            help(n, choose(&target, val, target_sum), target_sum)
        }
    };

    a + b + c
}

fn choose(map: &HashMap<i64, i32>, val: i64, target_sum: i64)->HashMap<i64,i32> {
    let mut res:HashMap<i64,i32>  = map.iter().map(|(&k, &v)| (k - val, v)).collect();
    res.entry(target_sum).and_modify(|x| *x += 1).or_insert(1);
    res
}
