package io.github.snow.link;

/*
83. 删除排序链表中的重复元素
已解答
简单
相关标签
相关企业
给定一个已排序的链表的头 head ， 删除所有重复的元素，使每个元素只出现一次 。返回 已排序的链表 。



示例 1：


输入：head = [1,1,2]
输出：[1,2]
示例 2：


输入：head = [1,1,2,3,3]
输出：[1,2,3]


提示：

链表中节点数目在范围 [0, 300] 内
-100 <= Node.val <= 100
题目数据保证链表已经按升序 排列
 */


/**
 * @author snow
 * @since 2024/1/14
 */
public class DeleteDuplicates {
    static class Solution {
        public ListNode deleteDuplicates(ListNode head) {
            ListNode dummy = new ListNode(Integer.MIN_VALUE);
            dummy.next = head;
            ListNode p = head;
            ListNode pre = dummy;
            while (p != null) {
                if (p.val == pre.val) {
                    pre.next = p.next;
                } else {
                    pre = p;
                }
                p = p.next;
            }
            return dummy.next;
        }
    }
}
