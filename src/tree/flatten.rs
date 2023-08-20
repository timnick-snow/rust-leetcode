#![allow(dead_code)]
/*
114. 二叉树展开为链表
提示
中等
1.5K
相关企业
给你二叉树的根结点 root ，请你将它展开为一个单链表：

展开后的单链表应该同样使用 TreeNode ，其中 right 子指针指向链表中下一个结点，而左子指针始终为 null 。
展开后的单链表应该与二叉树 先序遍历 顺序相同。


示例 1：


输入：root = [1,2,5,3,4,null,6]
输出：[1,null,2,null,3,null,4,null,5,null,6]
示例 2：

输入：root = []
输出：[]
示例 3：

输入：root = [0]
输出：[0]


提示：

树中结点数在范围 [0, 2000] 内
-100 <= Node.val <= 100


进阶：你可以使用原地算法（O(1) 额外空间）展开这棵树吗？
 */
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let root = root.as_ref().map_or(None, |x| Some(Rc::clone(x)));
        helper(root);
    }
}

fn helper(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let left = root.as_ref().unwrap().borrow().left.as_ref()
        .map_or(None, |x| Some(Rc::clone(x)));
    let right = root.as_ref().unwrap().borrow().right.as_ref()
        .map_or(None, |x| Some(Rc::clone(x)));

    match (left, right) {
        (None, None) => root,
        (None, right) => helper(right),
        (left, None) => {
            root.as_mut().unwrap().borrow_mut().left = None;
            root.as_mut().unwrap().borrow_mut().right = left;
            helper(
                root.as_ref().unwrap().borrow().right.as_ref()
                    .map_or(None, |x| Some(Rc::clone(x)))
            )
        }
        (left, right) => {
            // 交换左右子树
            root.as_mut().unwrap().borrow_mut().left = right;
            root.as_mut().unwrap().borrow_mut().right = left;
            // 递归处理右子树 返回右子树的最后节点
            let mut last = helper(
                root.as_ref().unwrap().borrow().right.as_ref()
                    .map_or(None, |x| Some(Rc::clone(x)))
            );
            // 最后节点连接左子树
            last.as_mut().unwrap().borrow_mut().right =
                root.as_ref().unwrap().borrow().left.as_ref()
                    .map_or(None, |x| Some(Rc::clone(x)));
            // 左子树置空
            root.as_mut().unwrap().borrow_mut().left = None;
            // 继续处理剩余节点
            helper(
                last.unwrap().borrow().right.as_ref()
                    .map_or(None, |x| Some(Rc::clone(x)))
            )
        }
    }
}