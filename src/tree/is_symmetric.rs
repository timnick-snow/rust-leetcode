#![allow(dead_code)]
/*
101. 对称二叉树
简单
2.5K
相关企业
给你一个二叉树的根节点 root ， 检查它是否轴对称。



示例 1：


输入：root = [1,2,2,3,4,4,3]
输出：true
示例 2：


输入：root = [1,2,2,null,3,null,3]
输出：false


提示：

树中节点数目在范围 [1, 1000] 内
-100 <= Node.val <= 100


进阶：你可以运用递归和迭代两种方法解决这个问题吗？
 */
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn is_symmetric_rec(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        helper(root.as_ref().unwrap().borrow().left.as_ref(), root.as_ref().unwrap().borrow().right.as_ref())
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut deque = VecDeque::new();
        // 将 root.left root.right 添加到队列
        deque.push_back(root.as_ref().unwrap().borrow().left.as_ref().map_or(None, |x| Some(Rc::clone(x))));
        deque.push_back(root.as_ref().unwrap().borrow().right.as_ref().map_or(None, |x| Some(Rc::clone(x))));
        while !deque.is_empty() {
            let p = deque.pop_back().unwrap();
            let q = deque.pop_back().unwrap();
            match (p, q) {
                (Some(n1), Some(n2)) => {
                    if n1.borrow().val != n2.borrow().val {
                        return false;
                    } else {
                        deque.push_back(n1.borrow().left.as_ref().map_or(None, |x| Some(Rc::clone(x))));
                        deque.push_back(n2.borrow().right.as_ref().map_or(None, |x| Some(Rc::clone(x))));
                        deque.push_back(n1.borrow().right.as_ref().map_or(None, |x| Some(Rc::clone(x))));
                        deque.push_back(n2.borrow().left.as_ref().map_or(None, |x| Some(Rc::clone(x))));
                    }
                }
                (None, None) => (),
                (_, _) => {
                    return false;
                }
            }
        }
        true
    }
}

fn helper(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(n1), Some(n2)) => {
            if n1.borrow().val != n2.borrow().val {
                false
            } else {
                helper(n1.borrow().left.as_ref(), n2.borrow().right.as_ref())
                    && helper(n1.borrow().right.as_ref(), n2.borrow().left.as_ref())
            }
        }
        (None, None) => true,
        (_, _) => false
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn t1() {
        println!("{}", "string");
    }
}

