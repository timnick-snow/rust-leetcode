package io.github.snow.link;

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
}
