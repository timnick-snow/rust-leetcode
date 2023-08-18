#![allow(dead_code)]

struct Solution;

use crate::tree::TreeNode;
/*
105. 从前序与中序遍历序列构造二叉树
中等
2K
相关企业
给定两个整数数组 preorder 和 inorder ，其中 preorder 是二叉树的先序遍历， inorder 是同一棵树的中序遍历，请构造二叉树并返回其根节点。



示例 1:
       3
    9     20
        15  7

输入: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
输出: [3,9,20,null,null,15,7]
示例 2:

输入: preorder = [-1], inorder = [-1]
输出: [-1]


提示:

1 <= preorder.length <= 3000
inorder.length == preorder.length
-3000 <= preorder[i], inorder[i] <= 3000
preorder 和 inorder 均 无重复 元素
inorder 均出现在 preorder
preorder 保证 为二叉树的前序遍历序列
inorder 保证 为二叉树的中序遍历序列
 */
use std::rc::Rc;
use std::cell::RefCell;

/*
以示例1为例
          根节点
preorder = [3,      9,      20,     15,     7]

          左子树     根节点           右子树
          |        |        |-------|-------|
          |        |        |-------|-------|
inorder = [9,       3,      15,     20,     7]


从先序遍历确定根节点，然后可以根据根节点在中序遍历中，将数组划分成两部分，分别为左子树和右子树
从而可以分解成小问题，递归求解

在inorder中找到根节点的索引p, 则左子树: &inorder[0..p]   右子树: &inorder[p+1..]

可以根据中序遍历中左子树的节点数目 划分先序遍历数组，中序中根节点索引为p，则左子树节点数量为p, 右子树节点数量为len-p-1
从而确定 先序遍历数组中的 左子树为: &preorder[1..p+1]   右子树: &preorder[p+1..]
 */
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        helper(&preorder[..], &inorder[..])
    }
}

fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.len() == 0 {
        return None;
    }
    let root_value = preorder[0];
    let p = inorder.iter().position(|&x| x == root_value).unwrap();

    Some(Rc::new(RefCell::new(
        TreeNode {
            val: root_value,
            left: helper(&preorder[1..p + 1], &inorder[0..p]),
            right: helper(&preorder[p + 1..], &inorder[p + 1..]),
        }
    )))
}