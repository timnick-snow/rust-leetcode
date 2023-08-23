package io.github.snow.tree;
/*
116. 填充每个节点的下一个右侧节点指针
中等
1K
相关企业
给定一个 完美二叉树 ，其所有叶子节点都在同一层，每个父节点都有两个子节点。二叉树定义如下：

struct Node {
  int val;
  Node *left;
  Node *right;
  Node *next;
}
填充它的每个 next 指针，让这个指针指向其下一个右侧节点。如果找不到下一个右侧节点，则将 next 指针设置为 NULL。

初始状态下，所有 next 指针都被设置为 NULL。



示例 1：



输入：root = [1,2,3,4,5,6,7]
输出：[1,#,2,3,#,4,5,6,7,#]
解释：给定二叉树如图 A 所示，你的函数应该填充它的每个 next 指针，以指向其下一个右侧节点，如图 B 所示。序列化的输出按层序遍历排列，同一层节点由 next 指针连接，'#' 标志着每一层的结束。
示例 2:

输入：root = []
输出：[]


提示：

树中节点的数量在 [0, 212 - 1] 范围内
-1000 <= node.val <= 1000


进阶：

你只能使用常量级额外空间。
使用递归解题也符合要求，本题中递归程序占用的栈空间不算做额外的空间复杂度。
 */

import java.util.ArrayDeque;

/**
 * @author snow
 * @since 2023/8/20
 */
public class ConnectNext {
    class Solution {
        public Node connect(Node root) {
            if (root == null) {
                return null;
            }
            ArrayDeque<Node> deque = new ArrayDeque<>();
            deque.addLast(root);
            while (!deque.isEmpty()) {
                ArrayDeque<Node> temp = new ArrayDeque<>();
                while (!deque.isEmpty()) {
                    Node node = deque.pollFirst();
                    if (node.left != null) {
                        node.left.next = node.right;
                        temp.addLast(node.left);
                    }
                    if (node.right != null) {
                        node.right.next = node.next == null ? null : node.next.left;
                        temp.addLast(node.right);
                    }
                }
                deque = temp;
            }
            return root;
        }
    }

    void test() {
        Node root = new Node(1);
        Node n2 = new Node(2);
        Node n3 = new Node(3);
        Node n4 = new Node(4);
        Node n5 = new Node(5);
        Node n6 = new Node(6);
        Node n7 = new Node(7);

        root.left = n2;
        root.right = n3;

        root.left.left = n4;
        root.left.right = n5;
        root.right.left = n6;
        root.right.right = n7;

        new Solution().connect(root);
        assert root.next == null;
        assert n2.next == n3;
        assert n4.next == n5;
        assert n5.next == n6;
        assert n6.next == n7;
    }

    public static void main(String[] args) {
        new ConnectNext().test();
    }

}
