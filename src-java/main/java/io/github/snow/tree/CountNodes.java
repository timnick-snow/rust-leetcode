package io.github.snow.tree;

/*
222. 完全二叉树的节点个数
中等
1K
相关企业
给你一棵 完全二叉树 的根节点 root ，求出该树的节点个数。

完全二叉树 的定义如下：在完全二叉树中，除了最底层节点可能没填满外，其余每层节点数都达到最大值，并且最下面一层的节点都集中在该层最左边的若干位置。若最底层为第 h 层，则该层包含 1~ 2h 个节点。



示例 1：


输入：root = [1,2,3,4,5,6]
输出：6
示例 2：

输入：root = []
输出：0
示例 3：

输入：root = [1]
输出：1


提示：

树中节点的数目范围是[0, 5 * 104]
0 <= Node.val <= 5 * 104
题目数据保证输入的树是 完全二叉树


进阶：遍历树来统计节点是一种时间复杂度为 O(n) 的简单解决方案。你可以设计一个更快的算法吗？
 */

import org.assertj.core.api.WithAssertions;
import org.junit.jupiter.api.Test;

/**
 * 222. 完全二叉树的节点个数
 *
 * @author snow
 * @since 2023/9/10
 */
public class CountNodes implements WithAssertions {
    /*
     * 二分查找 + 位运算
     *
     * 常规的遍历需要 O(n) 的时间复杂度
     * 由于树是完全二叉树，不断的遍历左孩子，可以得到树的最大层数 h （规定根结点的层数为0）
     * 那么 0到 h-1 层的节点是满的，共有 2^h-1 个节点
     * 最底层最多有 2^h 个节点，节点序号为 2^h ~ 2^(h+1)-1
     *
     *
     */
    static class Solution {
        public int countNodes(TreeNode root) {
            if (root == null) {
                return 0;
            }
            int h = 0;
            TreeNode p = root;
            while (p.left != null) {
                p = p.left;
                h++;
            }
            // 二分查找
            int low = 1 << h;
            int high = (1 << (h + 1)) - 1;
            while (low < high) {
                int mid = (low + high + 1) >> 1;
                if (check(root, h, mid)) {
                    low = mid;
                } else {
                    high = mid - 1;
                }
            }
            return low;
        }

        private boolean check(TreeNode root, int h, int k) {
            TreeNode node = root;
            for (int i = h - 1; i >= 0; i--) {
                if ((k & (1 << i)) == 0) {
                    node = node.left;
                } else {
                    node = node.right;
                }
            }
            return node != null;
        }
    }

    private static final Solution solution = new Solution();

    @Test
    public void fun1() {
        TreeNode root = new TreeNode(1);
        TreeNode n2 = new TreeNode(2);
        TreeNode n3 = new TreeNode(3);
        TreeNode n4 = new TreeNode(4);
        TreeNode n5 = new TreeNode(5);
        TreeNode n6 = new TreeNode(6);

        root.left = n2;
        root.right = n3;
        n2.left = n4;
        n2.right = n5;
        n3.left = n6;

        int count = solution.countNodes(root);
        assertThat(count).isEqualTo(6);
    }
}
