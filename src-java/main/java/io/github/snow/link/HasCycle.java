package io.github.snow.link;
/*
141. 环形链表
简单
2K
相关企业
给你一个链表的头节点 head ，判断链表中是否有环。

如果链表中有某个节点，可以通过连续跟踪 next 指针再次到达，则链表中存在环。 为了表示给定链表中的环，评测系统内部使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。注意：pos 不作为参数进行传递 。仅仅是为了标识链表的实际情况。

如果链表中存在环 ，则返回 true 。 否则，返回 false 。



示例 1：



输入：head = [3,2,0,-4], pos = 1
输出：true
解释：链表中有一个环，其尾部连接到第二个节点。
示例 2：



输入：head = [1,2], pos = 0
输出：true
解释：链表中有一个环，其尾部连接到第一个节点。
示例 3：



输入：head = [1], pos = -1
输出：false
解释：链表中没有环。


提示：

链表中节点的数目范围是 [0, 104]
-105 <= Node.val <= 105
pos 为 -1 或者链表中的一个 有效索引 。


进阶：你能用 O(1)（即，常量）内存解决此问题吗？
 */

import org.assertj.core.api.WithAssertions;
import org.junit.jupiter.api.Test;

/**
 * 141. 环形链表
 *
 * @author snow
 * @since 2023/8/25
 */
public class HasCycle implements WithAssertions {
    /*
     * 快慢指针
     * fast指针每次移动2个距离，slow指针每次移动2个距离。
     * 快指针对于慢指针的相对速度为1。这样，如果存在环，fast和slow指针都将进入环内，fast指针将追上slow指针
     * 即fast，slow指针会相遇
     */
    static class Solution {
        public boolean hasCycle(ListNode head) {
            if (head == null) {
                return false;
            }
            ListNode fast = head, slow = head;
            while (fast != null && fast.next != null) {
                fast = fast.next.next;
                slow = slow.next;
                if (fast == slow) {
                    return true;
                }
            }
            return false;
        }
    }

    private final Solution solution = new Solution();

    @Test
    public void example01() {
        ListNode head = new ListNode(3);
        ListNode n1 = new ListNode(2);
        ListNode n2 = new ListNode(0);
        ListNode n3 = new ListNode(-4);
        head.next = n1;
        n1.next = n2;
        n2.next = n3;

        n3.next = n1;

        assertThat(solution.hasCycle(head)).isTrue();
    }

    @Test
    public void example02() {
        ListNode head = new ListNode(1);
        ListNode n1 = new ListNode(2);
        head.next = n1;
        n1.next = head;
        assertThat(solution.hasCycle(head)).isTrue();
    }

    @Test
    public void example03() {
        ListNode head = new ListNode(1);
        assertThat(solution.hasCycle(head)).isFalse();
    }
}
