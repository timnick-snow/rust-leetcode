package io.github.snow.tree;
/*
236. 二叉树的最近公共祖先
中等
相关标签
相关企业
给定一个二叉树, 找到该树中两个指定节点的最近公共祖先。

百度百科中最近公共祖先的定义为：“对于有根树 T 的两个节点 p、q，最近公共祖先表示为一个节点 x，满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点也可以是它自己的祖先）。”



示例 1：


输入：root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
输出：3
解释：节点 5 和节点 1 的最近公共祖先是节点 3 。
示例 2：


输入：root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
输出：5
解释：节点 5 和节点 4 的最近公共祖先是节点 5 。因为根据定义最近公共祖先节点可以为节点本身。
示例 3：

输入：root = [1,2], p = 1, q = 2
输出：1


提示：

树中节点数目在范围 [2, 105] 内。
-109 <= Node.val <= 109
所有 Node.val 互不相同 。
p != q
p 和 q 均存在于给定的二叉树中。
 */

import org.assertj.core.api.WithAssertions;

import java.util.ArrayList;

/**
 * 236. 二叉树的最近公共祖先
 *
 * @author snow
 * @since 2023/9/22
 */
public class LowestCommonAncestor2 implements WithAssertions {
    static class Solution {
        ArrayList<Integer> path1 = null;
        ArrayList<Integer> path2 = null;

        public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
            preOrder(root, p, q, new ArrayList<>());
            int minSize = Math.min(path1.size(), path2.size());
            TreeNode ans = root;
            for (int i = 0; i < minSize; i++) {
                int d1 = path1.get(i);
                int d2 = path2.get(i);
                if (d1 != d2) {
                    break;
                }
                if (d1 == 0) {
                    ans = ans.left;
                } else {
                    ans = ans.right;
                }
            }
            return ans;
        }

        private void preOrder(TreeNode root, TreeNode p, TreeNode q, ArrayList<Integer> path) {
            if (root == null || (path1 != null && path2 != null)) {
                return;
            }
            if (root.val == p.val) {
                path1 = new ArrayList<>(path);
            }
            if (root.val == q.val) {
                path2 = new ArrayList<>(path);
            }
            path.add(0);
            preOrder(root.left, p, q, path);
            path.remove(path.size() - 1);

            path.add(1);
            preOrder(root.right, p, q, path);
            path.remove(path.size() - 1);
        }
    }
}
