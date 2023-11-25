package io.github.snow.other;
/*
430. 扁平化多级双向链表
中等
相关标签
相关企业
你会得到一个双链表，其中包含的节点有一个下一个指针、一个前一个指针和一个额外的 子指针 。这个子指针可能指向一个单独的双向链表，也包含这些特殊的节点。这些子列表可以有一个或多个自己的子列表，以此类推，以生成如下面的示例所示的 多层数据结构 。

给定链表的头节点 head ，将链表 扁平化 ，以便所有节点都出现在单层双链表中。让 curr 是一个带有子列表的节点。子列表中的节点应该出现在扁平化列表中的 curr 之后 和 curr.next 之前 。

返回 扁平列表的 head 。列表中的节点必须将其 所有 子指针设置为 null 。



示例 1：



输入：head = [1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]
输出：[1,2,3,7,8,11,12,9,10,4,5,6]
解释：输入的多级列表如上图所示。
扁平化后的链表如下图：

示例 2：



输入：head = [1,2,null,3]
输出：[1,3,2]
解释：输入的多级列表如上图所示。
扁平化后的链表如下图：

示例 3：

输入：head = []
输出：[]
说明：输入中可能存在空列表。


提示：

节点数目不超过 1000
1 <= Node.val <= 105
 */

/**
 * @author snow
 * @since 2023/11/25
 */
public class FlattenLinkedList {

    static class Solution {
        public Node flatten(Node head) {
            if (head == null) {
                return null;
            }
            return help(head)[0];
        }

        private Node[] help(Node head) {
            Node cur = head;
            Node tail = null;
            while (cur != null) {
                Node next = cur.next;
                if (cur.child == null) {
                    tail = cur;
                } else {
                    // 连接children
                    Node[] ch = help(cur.child);
                    cur.next = ch[0];
                    ch[0].prev = cur;
                    ch[1].next = next;
                    if (next != null) {
                        next.prev = ch[1];
                    }

                    cur.child = null;
                    tail = ch[1];
                }
                cur = next;
            }

            return new Node[]{head, tail};
        }
    }

    static class Node {
        public int val;
        public Node prev;
        public Node next;
        public Node child;
    }
}
