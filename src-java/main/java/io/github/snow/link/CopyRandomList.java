/*
138. 复制带随机指针的链表
提示
中等
1.2K
相关企业
给你一个长度为 n 的链表，每个节点包含一个额外增加的随机指针 random ，该指针可以指向链表中的任何节点或空节点。

构造这个链表的 深拷贝。 深拷贝应该正好由 n 个 全新 节点组成，其中每个新节点的值都设为其对应的原节点的值。新节点的 next 指针和 random 指针也都应指向复制链表中的新节点，并使原链表和复制链表中的这些指针能够表示相同的链表状态。复制链表中的指针都不应指向原链表中的节点 。

例如，如果原链表中有 X 和 Y 两个节点，其中 X.random --> Y 。那么在复制链表中对应的两个节点 x 和 y ，同样有 x.random --> y 。

返回复制链表的头节点。

用一个由 n 个节点组成的链表来表示输入/输出中的链表。每个节点用一个 [val, random_index] 表示：

val：一个表示 Node.val 的整数。
random_index：随机指针指向的节点索引（范围从 0 到 n-1）；如果不指向任何节点，则为  null 。
你的代码 只 接受原链表的头节点 head 作为传入参数。



示例 1：



输入：head = [[7,null],[13,0],[11,4],[10,2],[1,0]]
输出：[[7,null],[13,0],[11,4],[10,2],[1,0]]
示例 2：



输入：head = [[1,1],[2,1]]
输出：[[1,1],[2,1]]
示例 3：



输入：head = [[3,null],[3,0],[3,null]]
输出：[[3,null],[3,0],[3,null]]


提示：

0 <= n <= 1000
-104 <= Node.val <= 104
Node.random 为 null 或指向链表中的节点。
 */

package io.github.snow.link;

import org.assertj.core.api.WithAssertions;
import org.junit.jupiter.api.Test;

/**
 * 138. 复制带随机指针的链表
 *
 * @author snow
 * @since 2023/8/25
 */
public class CopyRandomList implements WithAssertions {
    /*
     * 复制原链表中的每一个节点，并插入到原节点的后继节点中
     * 原链表：A -> B -> C -> D
     * 插入后：A -> A' -> B -> B' -> C -> C' -> D -> D'
     *
     * 插入后的链表是由原节点和复制节点交替形成的，每一个节点的复制品都是其后继，即我们有：copy(node) = node.next
     *
     * 我们处理随机指针
     * 对于原链表的节点 node, 其复制节点为 node.next，原节点的随机指针指向的节点 node.random 的复制品为 node.random.next
     * 于是只要让 node.next.random = node.random.next，就可以完成对复制节点的随机指针赋值
     * 注意 node.random 可能是null
     *
     * 最后我们解开交替链表，将所有复制品重新形成一个链表返回即可
     */
    static class Solution {
        public Node copyRandomList(Node head) {
            if (head == null) {
                return null;
            }
            // 复制每一个节点 形成交替链表
            Node p = head;
            while (p != null) {
                Node copy = new Node(p.val);
                copy.next = p.next;
                p.next = copy;
                p = copy.next;
            }
            // 处理复制节点的随机指针
            p = head;
            while (p != null) {
                p.next.random = p.random == null ? null : p.random.next;
                p = p.next.next;
            }
            // 解开交替链表
            Node dummy = new Node(0);
            p = head;
            Node q = dummy;
            while (p != null) {
                q.next = p.next;
                p.next = p.next.next;
                // 移动指针
                p = p.next;
                q = q.next;
            }
            return dummy.next;
        }
    }

    private final Solution solution = new Solution();

    @Test
    public void example01() {
        Node n0 = new Node(7);
        Node n1 = new Node(13);
        Node n2 = new Node(11);
        Node n3 = new Node(10);
        Node n4 = new Node(1);

        n0.next = n1;
        n1.next = n2;
        n3.next = n3;
        n4.next = n4;

        n1.random = n0;
        n2.random = n4;
        n3.random = n2;
        n4.random = n0;

        Node node = solution.copyRandomList(n0);
        assertThat(node.val).isEqualTo(n0.val);
        assertThat(node.random).isNull();

        assertThat(node.next.val).isEqualTo(n1.val);
        assertThat(node.next.random.val).isEqualTo(n1.random.val);
    }
}
