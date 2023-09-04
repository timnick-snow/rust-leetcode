package io.github.snow.tree;

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

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;

/**
 * 449. 序列化和反序列化二叉搜索树
 *
 * @author snow
 * @since 2023/9/4
 */
public class Codec {
    public String serialize(TreeNode root) {
        List<Integer> list = new ArrayList<>();
        postOrder(root, list);
        String str = list.toString();
        return str.substring(1, str.length() - 1);
    }

    public TreeNode deserialize(String data) {
        if (data.isEmpty()) {
            return null;
        }
        String[] arr = data.split(", ");
        ArrayDeque<Integer> deque = new ArrayDeque<>();
        for (String val : arr) {
            deque.push(Integer.parseInt(val));
        }
        return construct(Integer.MIN_VALUE, Integer.MAX_VALUE, deque);
    }

    /**
     * 后序遍历得到的数组中，根结点的值位于数组末尾
     * 左子树的节点均小于根节点的值，右子树的节点均大于根节点的值
     * 可以根据这些性质设计递归函数恢复二叉搜索树。
     */
    private TreeNode construct(int lower, int upper, ArrayDeque<Integer> deque) {
        if (deque.isEmpty() || deque.peek() < lower || deque.peek() > upper) {
            return null;
        }
        int val = deque.pop();
        TreeNode root = new TreeNode(val);
        root.right = construct(val, upper, deque);
        root.left = construct(lower, val, deque);
        return root;
    }

    private void postOrder(TreeNode root, List<Integer> list) {
        if (root == null) {
            return;
        }
        postOrder(root.left, list);
        postOrder(root.right, list);
        list.add(root.val);
    }
}
