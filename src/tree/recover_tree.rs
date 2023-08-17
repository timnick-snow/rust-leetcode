#![allow(dead_code)]
/*
99. 恢复二叉搜索树
中等
876
相关企业
给你二叉搜索树的根节点 root ，该树中的 恰好 两个节点的值被错误地交换。请在不改变其结构的情况下，恢复这棵树 。



示例 1：


输入：root = [1,3,null,null,2]
输出：[3,1,null,null,2]
解释：3 不能是 1 的左孩子，因为 3 > 1 。交换 1 和 3 使二叉搜索树有效。
示例 2：


输入：root = [3,1,4,null,null,2]
输出：[2,1,4,null,null,3]
解释：2 不能在 3 的右子树中，因为 2 < 3 。交换 2 和 3 使二叉搜索树有效。


提示：

树上节点的数目在范围 [2, 1000] 内
-231 <= Node.val <= 231 - 1


进阶：使用 O(n) 空间复杂度的解法很容易实现。你能想出一个只使用 O(1) 空间的解决方案吗？
 */
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::TreeNode;

struct Solution;

/*
使用中序遍历二叉搜索树将得到一个线性的递增序列

递增序列中交换两个节点值
交换相邻节点 [1,2,3,4,5,6]  => [1,2,4,3,5,6] , 将出现一个节点值3不满足递增，需要将其和前一个节点值互换
交换非相邻节点 [1,2,3,4,5,6]  => [1,5,3,4,2,6] , 将出现两个节点值3,2不满足递增，需要将第二个节点(2)和第一个节点的前节点(5)互换
 */
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = Vec::new();
        let (mut x, mut y) = (None, None);
        let mut pre: Option<Rc<RefCell<TreeNode>>> = None;

        let mut head = Some(Rc::clone(root.as_ref().unwrap()));
        while !stack.is_empty() || head.is_some() {
            // 中序遍历 先将所有左节点入栈  找到最左节点
            while let Some(node) = head {
                stack.push(Rc::clone(&node));
                head = node.borrow().left.as_ref().map_or(None, |x| Some(Rc::clone(x)));
            }
            // 已到最左节点 此节点没有左孩子 弹栈访问该节点
            let node = stack.pop().unwrap();
            // 判断该节点与前一个节点是否构成递增序列, 这里需要找到1~2次递减的位置
            if pre.is_some() && node.borrow().val < pre.as_ref().unwrap().borrow().val {
                y = Some(Rc::clone(&node));
                if x.is_none() {
                    x = pre;
                } else {
                    break;
                }
            }
            // pre指向刚刚访问过的节点
            pre = Some(Rc::clone(&node));
            // 访问右孩子
            head = node.borrow().right.as_ref().map_or(None, |x| Some(Rc::clone(x)));
        }
        // 交换x, y 节点的值
        let temp = x.as_ref().unwrap().borrow().val;
        x.unwrap().borrow_mut().val = y.as_ref().unwrap().borrow().val;
        y.unwrap().borrow_mut().val = temp;
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn t1() {
        println!("{}", "string");
    }
}

