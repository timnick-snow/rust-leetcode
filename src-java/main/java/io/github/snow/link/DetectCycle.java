package io.github.snow.link;
/*
142. 环形链表 II
中等
2.3K
相关企业
给定一个链表的头节点  head ，返回链表开始入环的第一个节点。 如果链表无环，则返回 null。

如果链表中有某个节点，可以通过连续跟踪 next 指针再次到达，则链表中存在环。 为了表示给定链表中的环，评测系统内部使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。如果 pos 是 -1，则在该链表中没有环。注意：pos 不作为参数进行传递，仅仅是为了标识链表的实际情况。

不允许修改 链表。



示例 1：



输入：head = [3,2,0,-4], pos = 1
输出：返回索引为 1 的链表节点
解释：链表中有一个环，其尾部连接到第二个节点。
示例 2：



输入：head = [1,2], pos = 0
输出：返回索引为 0 的链表节点
解释：链表中有一个环，其尾部连接到第一个节点。
示例 3：



输入：head = [1], pos = -1
输出：返回 null
解释：链表中没有环。


提示：

链表中节点的数目范围在范围 [0, 104] 内
-105 <= Node.val <= 105
pos 的值为 -1 或者链表中的一个有效索引


进阶：你是否可以使用 O(1) 空间解决此题？
 */


import org.assertj.core.api.WithAssertions;
import org.junit.jupiter.api.Test;

/**
 * 142. 环形链表 II
 *
 * @author snow
 * @since 2023/8/25
 */
public class DetectCycle implements WithAssertions {
    /*
     *    a                       相遇点
     * 口 -> 口 -> 口 -> 口 -> 口 -> 口 -> 口
     *             |_____________________|
     *                       b
     *
     * 如果存在环，则快慢指针将在环内某节点相遇，假设经过时间t后，快慢指针相遇，记相遇节点距环入口的距离为 m
     * 记环之前的节点数为a，环内节点数为b
     * d(fast) = 2t = a + m + ub
     * d(slow) = t = a + m + vb
     *
     * 可得，t = (u-v)b
     * 从而，m = (u-2v)b - a
     *
     * 当快慢指针遇时，相遇点继续前进a个位置可回到相遇点
     *
     */
    static class Solution {
        public ListNode detectCycle(ListNode head) {
            if (head == null) {
                return null;
            }
            ListNode fast = head, slow = head;
            while (fast != null && fast.next != null) {
                fast = fast.next.next;
                slow = slow.next;
                if (fast == slow) {
                    ListNode p = head;
                    while (p != slow) {
                        p = p.next;
                        slow = slow.next;
                    }
                    return p;
                }
            }
            return null;
        }
    }

    private final Solution solution = new Solution();

    @Test
    public void example01() {
        ListNode head = ListNode.build(new int[]{3, 2, 0, -4});
        ListNode cycle = head.next;
        head.next.next.next.next = cycle;
        assertThat(solution.detectCycle(head)).isEqualTo(cycle);
    }
}
