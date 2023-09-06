package io.github.snow.link;

/*
203. 移除链表元素
简单
1.3K
相关企业
给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。


示例 1：


输入：head = [1,2,6,3,4,5,6], val = 6
输出：[1,2,3,4,5]
示例 2：

输入：head = [], val = 1
输出：[]
示例 3：

输入：head = [7,7,7,7], val = 7
输出：[]


提示：

列表中的节点数目在范围 [0, 104] 内
1 <= Node.val <= 50
0 <= val <= 50
 */

import org.assertj.core.api.WithAssertions;
import org.junit.jupiter.api.Test;

/**
 * 203. 移除链表元素
 *
 * @author snow
 * @since 2023/9/6
 */
public class RemoveElements implements WithAssertions {
    static class Solution {
        public ListNode removeElements(ListNode head, int val) {
            ListNode dummy = new ListNode(0);
            dummy.next = head;

            ListNode p = dummy;
            while (p.next != null) {
                if (p.next.val == val) {
                    // 移除下一个节点
                    p.next = p.next.next;
                } else {
                    p = p.next;
                }
            }

            return dummy.next;
        }
    }

    private static final Solution solution = new Solution();

    @Test
    public void fun1() throws Exception {

    }
}