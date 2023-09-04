#![allow(dead_code)]
/*
449. 序列化和反序列化二叉搜索树
中等
448
相关企业
序列化是将数据结构或对象转换为一系列位的过程，以便它可以存储在文件或内存缓冲区中，或通过网络连接链路传输，以便稍后在同一个或另一个计算机环境中重建。

设计一个算法来序列化和反序列化 二叉搜索树 。 对序列化/反序列化算法的工作方式没有限制。 您只需确保二叉搜索树可以序列化为字符串，并且可以将该字符串反序列化为最初的二叉搜索树。

编码的字符串应尽可能紧凑。



示例 1：

输入：root = [2,1,3]
输出：[2,1,3]
示例 2：

输入：root = []
输出：[]


提示：

树中节点数范围是 [0, 104]
0 <= Node.val <= 104
题目数据 保证 输入的树是一棵二叉搜索树。
 */

use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

struct Codec {}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut list = Vec::new();
        post_order(&root, &mut list);
        let mut ans = String::new();
        for x in list.into_iter() {
            ans.push_str(&x.to_string());
            ans.push(',');
        }
        ans
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let mut stack = Vec::new();
        for x in data.split(',') {
            if let Ok(val) = x.parse::<i32>(){
                stack.push(val);
            }
        }
        return construct(i32::MIN, i32::MAX, &mut stack);
    }
}

/**
 * 后序遍历得到的数组中，根结点的值位于数组末尾
 * 左子树的节点均小于根节点的值，右子树的节点均大于根节点的值
 * 可以根据这些性质设计递归函数恢复二叉搜索树。
 */
fn construct(lower: i32, upper: i32, stack: &mut Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if stack.is_empty() || *stack.last().unwrap() < lower || *stack.last().unwrap() > upper {
        return None;
    }
    let val = stack.pop().unwrap();
    let root = Rc::new(RefCell::new(TreeNode::new(val)));
    root.borrow_mut().right = construct(val, upper, stack);
    root.borrow_mut().left = construct(lower, val, stack);
    Some(root)
}

fn post_order(root: &Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }
    let left = &root.as_ref().unwrap().borrow().left;
    let right = &root.as_ref().unwrap().borrow().right;
    post_order(left, list);
    post_order(right, list);
    list.push(root.as_ref().unwrap().borrow().val);
}