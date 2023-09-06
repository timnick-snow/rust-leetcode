package io.github.snow.tree;

/*
1123. 最深叶节点的最近公共祖先
提示
中等
178
相关企业
给你一个有根节点 root 的二叉树，返回它 最深的叶节点的最近公共祖先 。

回想一下：

叶节点 是二叉树中没有子节点的节点
树的根节点的 深度 为 0，如果某一节点的深度为 d，那它的子节点的深度就是 d+1
如果我们假定 A 是一组节点 S 的 最近公共祖先，S 中的每个节点都在以 A 为根节点的子树中，且 A 的深度达到此条件下可能的最大值。


示例 1：


输入：root = [3,5,1,6,2,0,8,null,null,7,4]
输出：[2,7,4]
解释：我们返回值为 2 的节点，在图中用黄色标记。
在图中用蓝色标记的是树的最深的节点。
注意，节点 6、0 和 8 也是叶节点，但是它们的深度是 2 ，而节点 7 和 4 的深度是 3 。
示例 2：

输入：root = [1]
输出：[1]
解释：根节点是树中最深的节点，它是它本身的最近公共祖先。
示例 3：

输入：root = [0,1,3,null,2]
输出：[2]
解释：树中最深的叶节点是 2 ，最近公共祖先是它自己。


提示：

树中的节点数将在 [1, 1000] 的范围内。
0 <= Node.val <= 1000
每个节点的值都是 独一无二 的。


注意：本题与力扣 865 重复：https://leetcode-cn.com/problems/smallest-subtree-with-all-the-deepest-nodes/
 */

import org.assertj.core.api.WithAssertions;
import org.junit.jupiter.api.Test;

import java.util.AbstractMap;
import java.util.Map;

/**
 * 1123. 最深叶节点的最近公共祖先
 *
 * @author snow
 * @since 2023/9/6
 */
public class LcaDeepestLeaves implements WithAssertions {
    static class Solution {

        public TreeNode lcaDeepestLeaves(TreeNode root) {
            return lca(root).getKey();
        }

        private Map.Entry<TreeNode, Integer> lca(TreeNode root) {
            if (root == null) {
                return new AbstractMap.SimpleEntry<>(null, 0);
            }
            Map.Entry<TreeNode, Integer> left = lca(root.left);
            Map.Entry<TreeNode, Integer> right = lca(root.right);
            if (left.getValue() > right.getValue()) {
                return new AbstractMap.SimpleEntry<>(left.getKey(), left.getValue() + 1);
            }
            if (right.getValue() > left.getValue()) {
                return new AbstractMap.SimpleEntry<>(right.getKey(), right.getValue() + 1);
            }
            return new AbstractMap.SimpleEntry<>(root, left.getValue() + 1);
        }
    }

    private static Solution solution = new Solution();

    @Test
    public void fun1() throws Exception {
        TreeNode root = new TreeNode(3);
        TreeNode n5 = new TreeNode(5);
        TreeNode n1 = new TreeNode(1);
        TreeNode n6 = new TreeNode(6);
        TreeNode n2 = new TreeNode(2);
        TreeNode n0 = new TreeNode(0);
        TreeNode n8 = new TreeNode(8);
        TreeNode n10 = new TreeNode(10);
        TreeNode n7 = new TreeNode(7);
        TreeNode n4 = new TreeNode(4);

        root.left = n5;
        root.right = n1;
        n5.left = n6;
        n5.right = n2;
        n1.left = n0;
        n1.right = n8;
        n6.right = n10;
        n2.left = n7;
        n2.right = n4;

        TreeNode node = solution.lcaDeepestLeaves(root);
        System.out.println(node.val);
    }
}
