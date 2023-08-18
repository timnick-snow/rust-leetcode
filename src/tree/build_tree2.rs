#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

/*
106. 从中序与后序遍历序列构造二叉树
中等
1.1K
相关企业
给定两个整数数组 inorder 和 postorder ，其中 inorder 是二叉树的中序遍历， postorder 是同一棵树的后序遍历，请你构造并返回这颗 二叉树 。



示例 1:


输入：inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
输出：[3,9,20,null,null,15,7]
示例 2:

输入：inorder = [-1], postorder = [-1]
输出：[-1]


提示:

1 <= inorder.length <= 3000
postorder.length == inorder.length
-3000 <= inorder[i], postorder[i] <= 3000
inorder 和 postorder 都由 不同 的值组成
postorder 中每一个值都在 inorder 中
inorder 保证是树的中序遍历
postorder 保证是树的后序遍历
 */
/*
以示例1为例

          左子树     根节点           右子树
          |        |        |-------|-------|
          |        |        |-------|-------|
inorder = [9,       3,      15,     20,     7]

                                           根节点
postorder = [9,      15,      7,     20,     3]


后序遍历确定根节点，然后可以根据根节点在中序遍历中，将数组划分成两部分，分别为左子树和右子树
从而可以分解成小问题，递归求解

在inorder中找到根节点的索引p, 则左子树: &inorder[0..p]   右子树: &inorder[p+1..]

可以根据中序遍历中左子树的节点数目 划分先序遍历数组，中序中根节点索引为p，则左子树节点数量为p, 右子树节点数量为len-p-1
从而确定 后序遍历数组中的 左子树为: &postorder[0..p]   右子树: &postorder[p+1..]
 */
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        helper(&inorder[..], &postorder[..])
    }
}

fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let len = inorder.len();
    if len == 0 {
        return None;
    }
    let root_value = postorder[len - 1];
    let p = inorder.iter().position(|&x| x == root_value).unwrap();

    Some(Rc::new(RefCell::new(
        TreeNode {
            val: root_value,
            left: helper(&inorder[0..p], &postorder[0..p]),
            right: helper(&inorder[p + 1..], &postorder[p..len - 1]),
        }
    )))
}