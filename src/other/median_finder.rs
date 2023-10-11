#![allow(dead_code)]
/*
295. 数据流的中位数
困难
相关标签
相关企业
中位数是有序整数列表中的中间值。如果列表的大小是偶数，则没有中间值，中位数是两个中间值的平均值。

例如 arr = [2,3,4] 的中位数是 3 。
例如 arr = [2,3] 的中位数是 (2 + 3) / 2 = 2.5 。
实现 MedianFinder 类:

MedianFinder() 初始化 MedianFinder 对象。

void addNum(int num) 将数据流中的整数 num 添加到数据结构中。

double findMedian() 返回到目前为止所有元素的中位数。与实际答案相差 10-5 以内的答案将被接受。

示例 1：

输入
["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
[[], [1], [2], [], [3], []]
输出
[null, null, null, 1.5, null, 2.0]

解释
MedianFinder medianFinder = new MedianFinder();
medianFinder.addNum(1);    // arr = [1]
medianFinder.addNum(2);    // arr = [1, 2]
medianFinder.findMedian(); // 返回 1.5 ((1 + 2) / 2)
medianFinder.addNum(3);    // arr[1, 2, 3]
medianFinder.findMedian(); // return 2.0
提示:

-105 <= num <= 105
在调用 findMedian 之前，数据结构中至少有一个元素
最多 5 * 104 次调用 addNum 和 findMedian
 */

/*
双优先队列

larger: 小顶堆，保存较大的一半的数
smaller: 大顶堆，保存较小的一半的数

当N为偶数时，两个队列的堆顶元素即为中间的两个数，取平均值即为中位数
当N为奇数时，我们规定larger队列多存储一个数，那么这个队列的堆顶元素就是中位数

如何插入元素？ add(num)
1. N为偶数，需要往larger队列插入一个元素，先将num插入到smaller队列，再将smaller堆顶元素移除并添加到larger
2. N为奇数，需要往smaller队列插入一个元素，先将num插入到larger队列，再将larger堆顶元素移除并添加到smaller


 */

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    larger: BinaryHeap<Reverse<i32>>,
    smaller: BinaryHeap<i32>,
}


impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            larger: BinaryHeap::new(),
            smaller: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.larger.len() == self.smaller.len() {
            self.smaller.push(num);
            self.larger.push(Reverse(self.smaller.pop().unwrap()));
        } else {
            self.larger.push(Reverse(num));
            self.smaller.push(self.larger.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        if self.larger.len() == self.smaller.len() {
            (self.larger.peek().unwrap().0 as f64 + *self.smaller.peek().unwrap() as f64) / 2_f64
        } else {
            self.larger.peek().unwrap().0 as f64
        }
    }
}

/*
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */