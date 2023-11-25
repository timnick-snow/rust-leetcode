package io.github.snow.other;
/*
429. N 叉树的层序遍历
中等
相关标签
相关企业
给定一个 N 叉树，返回其节点值的层序遍历。（即从左到右，逐层遍历）。

树的序列化输入是用层序遍历，每组子节点都由 null 值分隔（参见示例）。



示例 1：



输入：root = [1,null,3,2,4,null,5,6]
输出：[[1],[3,2,4],[5,6]]
示例 2：



输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
输出：[[1],[2,3,4,5],[6,7,8,9,10],[11,12,13],[14]]


提示：

树的高度不会超过 1000
树的节点总数在 [0, 10^4] 之间
 */

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
/*
429. N 叉树的层序遍历
中等
相关标签
相关企业
给定一个 N 叉树，返回其节点值的层序遍历。（即从左到右，逐层遍历）。

树的序列化输入是用层序遍历，每组子节点都由 null 值分隔（参见示例）。



示例 1：



输入：root = [1,null,3,2,4,null,5,6]
输出：[[1],[3,2,4],[5,6]]
示例 2：



输入：root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
输出：[[1],[2,3,4,5],[6,7,8,9,10],[11,12,13],[14]]


提示：

树的高度不会超过 1000
树的节点总数在 [0, 10^4] 之间
 */

/**
 * 429. N 叉树的层序遍历
 *
 * @author snow
 * @since 2023/11/25
 */
public class NTreeLevelOrder {
    static class Node {
        public int val;
        public List<Node> children;

        public Node() {
        }

        public Node(int _val) {
            val = _val;
        }

        public Node(int _val, List<Node> _children) {
            val = _val;
            children = _children;
        }
    }

    static class Solution {
        public List<List<Integer>> levelOrder(Node root) {
            if (root == null) {
                return Collections.emptyList();
            }
            List<List<Integer>> ans = new ArrayList<>();

            ArrayDeque<Node> deque = new ArrayDeque<>();
            deque.addLast(root);

            while (!deque.isEmpty()) {
                ArrayDeque<Node> temp = new ArrayDeque<>();
                List<Integer> level = new ArrayList<>();
                while (!deque.isEmpty()) {
                    Node node = deque.pop();
                    level.add(node.val);
                    temp.addAll(node.children);
                }
                ans.add(level);
                deque = temp;
            }
            return ans;
        }
    }
}
