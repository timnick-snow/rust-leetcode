#![allow(dead_code)]
/*
1670. 设计前中后队列
中等
相关标签
相关企业
提示
请你设计一个队列，支持在前，中，后三个位置的 push 和 pop 操作。

请你完成 FrontMiddleBack 类：

FrontMiddleBack() 初始化队列。
void pushFront(int val) 将 val 添加到队列的 最前面 。
void pushMiddle(int val) 将 val 添加到队列的 正中间 。
void pushBack(int val) 将 val 添加到队里的 最后面 。
int popFront() 将 最前面 的元素从队列中删除并返回值，如果删除之前队列为空，那么返回 -1 。
int popMiddle() 将 正中间 的元素从队列中删除并返回值，如果删除之前队列为空，那么返回 -1 。
int popBack() 将 最后面 的元素从队列中删除并返回值，如果删除之前队列为空，那么返回 -1 。
请注意当有 两个 中间位置的时候，选择靠前面的位置进行操作。比方说：

将 6 添加到 [1, 2, 3, 4, 5] 的中间位置，结果数组为 [1, 2, 6, 3, 4, 5] 。
从 [1, 2, 3, 4, 5, 6] 的中间位置弹出元素，返回 3 ，数组变为 [1, 2, 4, 5, 6] 。


示例 1：

输入：
["FrontMiddleBackQueue", "pushFront", "pushBack", "pushMiddle", "pushMiddle", "popFront", "popMiddle", "popMiddle", "popBack", "popFront"]
[[], [1], [2], [3], [4], [], [], [], [], []]
输出：
[null, null, null, null, null, 1, 3, 4, 2, -1]

解释：
FrontMiddleBackQueue q = new FrontMiddleBackQueue();
q.pushFront(1);   // [1]
q.pushBack(2);    // [1, 2]
q.pushMiddle(3);  // [1, 3, 2]
q.pushMiddle(4);  // [1, 4, 3, 2]
q.popFront();     // 返回 1 -> [4, 3, 2]
q.popMiddle();    // 返回 3 -> [4, 2]
q.popMiddle();    // 返回 4 -> [2]
q.popBack();      // 返回 2 -> []
q.popFront();     // 返回 -1 -> [] （队列为空）


提示：

1 <= val <= 109
最多调用 1000 次 pushFront， pushMiddle， pushBack， popFront， popMiddle 和 popBack 。
*/
use std::collections::VecDeque;

#[derive(Debug)]
struct FrontMiddleBackQueue {
    first: VecDeque<i32>,
    second: VecDeque<i32>,
    balance: bool,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    fn new() -> Self {
        FrontMiddleBackQueue {
            first: VecDeque::new(),
            second: VecDeque::new(),
            balance: true,
        }
    }

    fn push_front(&mut self, val: i32) {
        self.first.push_front(val);
        self.first_to_second(true);
    }

    fn push_middle(&mut self, val: i32) {
        self.first_to_second(true);
        self.first.push_back(val);
    }

    fn push_back(&mut self, val: i32) {
        self.second.push_back(val);
        self.second_to_first(true);
    }

    fn pop_front(&mut self) -> i32 {
        let val = self.first.pop_front().unwrap_or(-1);
        self.second_to_first(val != -1);
        val
    }

    fn pop_middle(&mut self) -> i32 {
        let val = self.first.pop_back().unwrap_or(-1);
        self.second_to_first(val != -1);
        val
    }

    fn pop_back(&mut self) -> i32 {
        let val = self.second.pop_back()
            .unwrap_or_else(|| self.first.pop_back().unwrap_or(-1));
        self.first_to_second(val != -1);
        val
    }

    fn first_to_second(&mut self, toggle: bool) {
        if !self.balance {
            if let Some(x) = self.first.pop_back() {
                self.second.push_front(x);
            }
        }
        if toggle {
            self.balance = !self.balance;
        }
    }

    fn second_to_first(&mut self, toggle: bool) {
        if self.balance {
            if let Some(x) = self.second.pop_front() {
                self.first.push_back(x);
            }
        }
        if toggle {
            self.balance = !self.balance;
        }
    }
}

/*
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */

#[cfg(test)]
mod test {
    use crate::other::front_middle_back_queue::FrontMiddleBackQueue;

    #[test]
    pub fn t1() {
        let mut queue = FrontMiddleBackQueue::new();
        assert_eq!(queue.pop_middle(), -1);
        queue.push_middle(5422);
        queue.push_middle(532228);
        assert_eq!(queue.pop_back(), 5422);
    }
}