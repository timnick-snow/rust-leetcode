#![allow(dead_code)]
/*
155. 最小栈
提示
中等
1.6K
相关企业
设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。

实现 MinStack 类:

MinStack() 初始化堆栈对象。
void push(int val) 将元素val推入堆栈。
void pop() 删除堆栈顶部的元素。
int top() 获取堆栈顶部的元素。
int getMin() 获取堆栈中的最小元素。


示例 1:

输入：
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]

输出：
[null,null,null,null,-3,null,0,-2]

解释：
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin();   --> 返回 -3.
minStack.pop();
minStack.top();      --> 返回 0.
minStack.getMin();   --> 返回 -2.


提示：

-231 <= val <= 231 - 1
pop、top 和 getMin 操作总是在 非空栈 上调用
push, pop, top, and getMin最多被调用 3 * 104 次




 Your MinStack object will be instantiated and called as such:
 let obj = MinStack::new();
 obj.push(val);
 obj.pop();
 let ret_3: i32 = obj.top();
 let ret_4: i32 = obj.get_min();
*/
struct MinStack {
    min_value: i64,
    stack: Vec<i64>,
}

/*
保存差值

top = min_value + diff

将diff保存到栈中, diff = top - min_value

diff >= 0, top > min_value, 最小值不会发生变化
    top = min_value + diff
    pop时最小值不变

diff < 0, top < min_value, 最小值将发生变化(变得更小)
    top = min_value
    pop时，最小值将变大 next(min_value) = min_value - diff

差值可能溢出 i32
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            min_value: 0,
            stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push(0);
            self.min_value = val as i64;
        } else {
            let diff = val as i64 - self.min_value;
            self.stack.push(diff);
            self.min_value = std::cmp::min(self.min_value, val as i64);
        }
    }

    fn pop(&mut self) {
        let diff = self.stack.pop().unwrap();
        if diff < 0 {
            self.min_value = self.min_value - diff;
        }
    }

    fn top(&self) -> i32 {
        let diff = *self.stack.last().unwrap();
        if diff < 0 {
            self.min_value as i32
        } else {
            (self.min_value + diff) as i32
        }
    }

    fn get_min(&self) -> i32 {
        self.min_value as i32
    }
}
