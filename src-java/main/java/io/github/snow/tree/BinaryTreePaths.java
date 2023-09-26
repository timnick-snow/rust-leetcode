package io.github.snow.tree;

/*
257. 二叉树的所有路径
简单
相关标签
相关企业
给你一个二叉树的根节点 root ，按 任意顺序 ，返回所有从根节点到叶子节点的路径。

叶子节点 是指没有子节点的节点。


示例 1：


输入：root = [1,2,3,null,5]
输出：["1->2->5","1->3"]
示例 2：

输入：root = [1]
输出：["1"]


提示：

树中节点的数目在范围 [1, 100] 内
-100 <= Node.val <= 100
 */

import java.util.ArrayList;
import java.util.List;

/**
 * 257. 二叉树的所有路径
 *
 * @author snow
 * @since 2023/9/26
 */
public class BinaryTreePaths {
    static class Solution {
        public List<String> binaryTreePaths(TreeNode root) {
            List<String> ans = new ArrayList<>();
            ArrayList<Integer> path = new ArrayList<>();
            path.add(root.val);
            dfs(root, path, ans);
            return ans;
        }

        public void dfs(TreeNode root, List<Integer> path, List<String> ans) {
            if (root.left == null && root.right == null) {
                ans.add(buildPath(path));
                return;
            }
            if (root.left != null) {
                path.add(root.left.val);
                dfs(root.left, path, ans);
                path.remove(path.size() - 1);
            }
            if (root.right != null) {
                path.add(root.right.val);
                dfs(root.right, path, ans);
                path.remove(path.size() - 1);
            }
        }

        private String buildPath(List<Integer> path) {
            StringBuilder sb = new StringBuilder();
            for (int value : path) {
                sb.append(value).append("->");
            }
            return sb.substring(0, sb.length() - 2);
        }
    }
}
