package io.github.snow.tree;
/*
199. 二叉树的右视图
中等
939
相关企业
给定一个二叉树的 根节点 root，想象自己站在它的右侧，按照从顶部到底部的顺序，返回从右侧所能看到的节点值。



示例 1:



输入: [1,2,3,null,5,null,4]
输出: [1,3,4]
示例 2:

输入: [1,null,3]
输出: [1,3]
示例 3:

输入: []
输出: []


提示:

二叉树的节点个数的范围是 [0,100]
-100 <= Node.val <= 100
 */

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

/**
 * 199. 二叉树的右视图
 *
 * @author snow
 * @since 2023/9/5
 */
public class RightSideView {
    /*
     * 层序遍历中，每层的最后一个节点就是能看到的节点
     */
    static class Solution {
        public List<Integer> rightSideView(TreeNode root) {
            if (root == null) {
                return Collections.emptyList();
            }
            // 队列
            ArrayDeque<TreeNode> deque = new ArrayDeque<>();
            deque.addLast(root);
            // 结果数组
            List<Integer> ans = new ArrayList<>();
            while (!deque.isEmpty()) {
                ArrayDeque<TreeNode> temp = new ArrayDeque<>();
                while (true) {
                    TreeNode node = deque.pop();
                    if (node.left != null) {
                        temp.addLast(node.left);
                    }
                    if (node.right != null) {
                        temp.addLast(node.right);
                    }
                    if (deque.isEmpty()) {
                        // 记录每层的最后一个值
                        ans.add(node.val);
                        break;
                    }
                }
                deque = temp;
            }
            return ans;
        }
    }
}
