package io.github.snow.link;

/**
 * 链表节点定义
 *
 * @author snow
 * @since 2023/8/25
 */
class Node {
    int val;
    Node next;
    Node random;

    public Node(int val) {
        this.val = val;
        this.next = null;
        this.random = null;
    }
}
