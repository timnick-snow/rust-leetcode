package io.github.snow.link;

import java.util.ArrayList;
import java.util.List;

/**
 * singly-linked list
 *
 * @author snow
 * @since 2023/8/25
 */
class ListNode {
    int val;
    ListNode next;

    ListNode(int x) {
        val = x;
        next = null;
    }

    static ListNode build(int[] arr) {
        ListNode dummy = new ListNode(0);
        ListNode p = dummy;
        for (int a : arr) {
            p.next = new ListNode(a);
            p = p.next;
        }
        return dummy.next;
    }

    public List<Integer> toList() {
        List<Integer> list = new ArrayList<>();
        ListNode p = this;
        while (p != null) {
            list.add(p.val);
            p = p.next;
        }
        return list;
    }
}
