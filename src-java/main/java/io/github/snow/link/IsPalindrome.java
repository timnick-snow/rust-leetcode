package io.github.snow.link;

/*
 234. 回文链表
简单
相关标签
相关企业
给你一个单链表的头节点 head ，请你判断该链表是否为回文链表。如果是，返回 true ；否则，返回 false 。



示例 1：


输入：head = [1,2,2,1]
输出：true
示例 2：


输入：head = [1,2]
输出：false


提示：

链表中节点数目在范围[1, 105] 内
0 <= Node.val <= 9


进阶：你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？
 */

import java.util.ArrayList;
import java.util.List;

/**
 * @author snow
 * @since 2023/9/22
 */
public class IsPalindrome {
    class Solution {
        public boolean isPalindrome(ListNode head) {
            ListNode p = head;
            List<Integer> list = new ArrayList<>();
            while (p != null) {
                list.add(p.val);
                p = p.next;
            }
            int left = 0, right = list.size() - 1;
            while (left < right) {
                if (list.get(left).intValue() != list.get(right).intValue()) {
                    return false;
                }
                left++;
                right--;
            }
            return true;
        }
    }
}
