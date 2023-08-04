#![allow(dead_code)]

use crate::link::ListNode;

// Definition for singly-linked list.
struct Solution {}

/*
给你两个非空 的链表，表示两个非负的整数。它们每位数字都是按照逆序的方式存储的，并且每个节点只能存储一位数字。
请你将两个数相加，并以相同形式返回一个表示和的链表。
你可以假设除了数字 0 之外，这两个数都不会以 0开头。

输入：l1 = [2,4,3], l2 = [5,6,4]
输出：[7,0,8]
解释：342 + 465 = 807.

输入：l1 = [0], l2 = [0]
输出：[0]

输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
输出：[8,9,9,9,0,0,0,1]

提示：

每个链表中的节点数在范围 [1, 100] 内
0 <= Node.val <= 9
题目数据保证列表表示的数字不含前导零

https://leetcode.cn/problems/add-two-numbers/
 */

type Linked = Option<Box<ListNode>>;

struct ResultLinked {
    result: Linked,
    next1: Linked,
    next2: Linked,
    carry: i32,
}

impl Solution {
    pub fn add_two_numbers2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let mut t = (l1, l2, 0, 0); // (list1, list2, value, carry)
        loop {
            t = match t {
                (None, None, _, 0) => break,
                (None, None, _, carry) => (None, None, carry, 0),
                (Some(l1), Some(l2), _, carry) => {
                    let sum = l1.val + l2.val + carry;
                    (l1.next, l2.next, sum % 10, sum / 10)
                }
                (Some(list), None, _, carry) => {
                    let sum = list.val + carry;
                    (list.next, None, sum % 10, sum / 10)
                }
                (None, Some(list), _, carry) => {
                    let sum = list.val + carry;
                    (None, list.next, sum % 10, sum / 10)
                }
            };

            *tail = Some(Box::new(ListNode::new(t.2)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        result
    }


    pub fn add_two_numbers(mut h1: Linked, mut h2: Linked) -> Linked {
        let mut answer = None;
        let mut tail = &mut answer;
        let mut carry = 0;
        while let ResultLinked {
            result: res @ Some(_),
            next1,
            next2,
            carry: res_carry
        } = Self::add_to_result(h1, h2, carry) {
            *tail = res;
            tail = &mut tail.as_mut().unwrap().next;

            h1 = next1;
            h2 = next2;
            carry = res_carry;
        }
        answer
    }

    fn add_to_result(n1: Linked, n2: Linked, mut carry: i32) -> ResultLinked {
        if n1.is_none() && n2.is_none() && carry == 0 {
            ResultLinked {
                result: None,
                next1: None,
                next2: None,
                carry: 0,
            }
        } else {
            ResultLinked {
                next1: n1.and_then(|x| {
                    carry += x.val;
                    x.next
                }),
                next2: n2.and_then(|x| {
                    carry += x.val;
                    x.next
                }),
                result: Some({
                    Box::new(ListNode::new(carry % 10))
                }),
                carry: carry / 10,
            }
        }
    }

    pub fn add_two_numbers1(h1: Option<Box<ListNode>>, h2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node1 = &h1.unwrap();
        let mut node2 = &h2.unwrap();
        let mut sum = node1.val + node2.val;
        let mut carry = sum / 10;
        let mut res = Box::new(ListNode::new(sum % 10));
        let mut node = &mut res;
        loop {
            sum = match (&node1.next, &node2.next) {
                (Some(n1), Some(n2)) => {
                    node1 = n1;
                    node2 = n2;
                    n1.val + n2.val + carry
                }
                (Some(n1), None) => {
                    node1 = n1;
                    n1.val + carry
                }
                (None, Some(n2)) => {
                    node2 = n2;
                    n2.val + carry
                }
                (None, None) => -1,
            };
            if sum == -1 {
                break;
            }
            // link res next node
            node.next = Some(Box::new(ListNode::new(sum % 10)));
            // update
            carry = sum / 10;
            node = node.next.as_mut().unwrap();
        }
        if carry > 0 {
            // link last carry
            node.next = Some(Box::new(ListNode::new(carry)));
        }
        Some(res)
    }
}

#[cfg(test)]
mod test {
    use crate::link::add_two_num::{ListNode, Solution};

    #[test]
    fn example1() {
        let h1 = ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(9))),
            })),
        };

        let h2 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(9))),
                })),
            })),
        };
        let res = Solution::add_two_numbers(Some(Box::new(h1)), Some(Box::new(h2)));
        println!("{:?}", res);
    }
}