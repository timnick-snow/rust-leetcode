#![allow(dead_code)]
/*
95. 不同的二叉搜索树 II
中等
1.5K
相关企业
给你一个整数 n ，请你生成并返回所有由 n 个节点组成且节点值从 1 到 n 互不相同的不同 二叉搜索树 。可以按 任意顺序 返回答案。



示例 1：


输入：n = 3
输出：[[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
示例 2：

输入：n = 1
输出：[[1]]


提示：

1 <= n <= 8
 */
struct Solution;

use crate::tree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;


impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        generate(1, n)
    }
}

fn generate(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if start > end {
        return vec![None];
    }
    if start == end {
        return vec![Some(Rc::new(RefCell::new(TreeNode::new(start))))];
    }
    let mut trees = Vec::new();
    // 枚举根节点
    for i in start..=end {
        // let root = Rc::new(RefCell::new(TreeNode::new(i)));
        let left_trees = generate(start, i - 1);
        let right_trees = generate(i + 1, end);
        for left_node in left_trees.into_iter() {
            for right_node in right_trees.iter() {
                let tree = Some(Rc::new(RefCell::new(TreeNode {
                    val: i,
                    left: left_node.clone(),
                    right: right_node.clone(),
                })));
                trees.push(tree);
            }
        }
    }
    trees
}