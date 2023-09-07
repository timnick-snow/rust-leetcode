package io.github.snow.link;

/*
206. 反转链表
简单
3.3K
相关企业
给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。


示例 1：


输入：head = [1,2,3,4,5]
输出：[5,4,3,2,1]
示例 2：


输入：head = [1,2]
输出：[2,1]
示例 3：

输入：head = []
输出：[]


提示：

链表中节点的数目范围是 [0, 5000]
-5000 <= Node.val <= 5000


进阶：链表可以选用迭代或递归方式完成反转。你能否用两种方法解决这道题？
 */

/**
 * 206. 反转链表
 *
 * @author snow
 * @since 2023/9/7
 */
public class ReverseList {
    static class Solution {
        public ListNode reverseList(ListNode head) {
            if (head == null) {
                return null;
            }
            ListNode cur = head;
            ListNode next = head.next;
            head.next = null;
            while (next != null) {
                ListNode temp = next.next;
                next.next = cur;
                cur = next;
                next = temp;
            }
            return cur;
        }
    }
}
