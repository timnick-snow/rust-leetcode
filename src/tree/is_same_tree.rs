#![allow(dead_code)]
/*
100. 相同的树
简单
1.1K
相关企业
给你两棵二叉树的根节点 p 和 q ，编写一个函数来检验这两棵树是否相同。

如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。



示例 1：


输入：p = [1,2,3], q = [1,2,3]
输出：true
示例 2：


输入：p = [1,2], q = [1,null,2]
输出：false
示例 3：


输入：p = [1,2,1], q = [1,1,2]
输出：false


提示：

两棵树上的节点数目都在范围 [0, 100] 内
-104 <= Node.val <= 104
 */
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        check_same(p.as_ref(), q.as_ref())
    }
}

fn check_same(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(n1), Some(n2)) => {
            if n1.borrow().val != n2.borrow().val {
                false
            } else {
                check_same(n1.borrow().left.as_ref(), n2.borrow().left.as_ref())
                    && check_same(n1.borrow().right.as_ref(), n2.borrow().right.as_ref())
            }
        }
        (None, None) => true,
        (_, _) => false
    }
}
