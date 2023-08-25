package io.github.snow.link;

/*
143. 重排链表
中等
1.4K
相关企业
给定一个单链表 L 的头节点 head ，单链表 L 表示为：

L0 → L1 → … → Ln - 1 → Ln
请将其重新排列后变为：

L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。



示例 1：



输入：head = [1,2,3,4]
输出：[1,4,2,3]
示例 2：



输入：head = [1,2,3,4,5]
输出：[1,5,2,4,3]


提示：

链表的长度范围为 [1, 5 * 104]
1 <= node.val <= 1000
 */

import org.assertj.core.api.WithAssertions;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

/**
 * 143. 重排链表
 *
 * @author snow
 * @since 2023/8/25
 */
public class ReorderList implements WithAssertions {
    static class Solution {
        public void reorderList(ListNode head) {
            List<ListNode> list = new ArrayList<>();
            ListNode p = head;
            while (p != null) {
                list.add(p);
                p = p.next;
            }
            for (int i = 0, j = list.size() - 1; i < j; i++, j--) {
                list.get(j).next = list.get(i).next;
                list.get(i).next = list.get(j);
            }
            list.get(list.size() >> 1).next = null;
        }
    }

    private final Solution solution = new Solution();

    @Test
    public void fun1()  {
        ListNode head = ListNode.build(new int[]{1, 2, 3, 4});
        solution.reorderList(head);
        List<Integer> list = head.toList();
        assertThat(list.get(0)).isEqualTo(1);
        assertThat(list.get(1)).isEqualTo(4);
        assertThat(list.get(2)).isEqualTo(2);
        assertThat(list.get(3)).isEqualTo(3);
    }
}
