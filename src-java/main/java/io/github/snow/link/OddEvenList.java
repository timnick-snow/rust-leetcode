package io.github.snow.link;
/*
328. 奇偶链表
中等
相关标签
相关企业
给定单链表的头节点 head ，将所有索引为奇数的节点和索引为偶数的节点分别组合在一起，然后返回重新排序的列表。

第一个节点的索引被认为是 奇数 ， 第二个节点的索引为 偶数 ，以此类推。

请注意，偶数组和奇数组内部的相对顺序应该与输入时保持一致。

你必须在 O(1) 的额外空间复杂度和 O(n) 的时间复杂度下解决这个问题。



示例 1:



输入: head = [1,2,3,4,5]
输出: [1,3,5,2,4]
示例 2:



输入: head = [2,1,3,5,6,4,7]
输出: [2,3,6,7,1,5,4]


提示:

n ==  链表中的节点数
0 <= n <= 104
-106 <= Node.val <= 106
 */

/**
 * 328. 奇偶链表
 *
 * @author snow
 * @since 2023/10/20
 */
public class OddEvenList {
    static class Solution {
        public ListNode oddEvenList(ListNode head) {
            if (head == null || head.next == null) {
                return head;
            }

            ListNode dummy = new ListNode(0);
            dummy.next = head;
            ListNode evenHead = head.next;
            // 奇偶指针
            ListNode p = head, q = head.next;
            while (q.next != null) {
                p.next = q.next;
                p = p.next;

                q.next = q.next.next;
                if (q.next != null) {
                    q = q.next;
                }
            }
            p.next = evenHead;
            return dummy.next;
        }
    }
}
