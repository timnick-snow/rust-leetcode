#![allow(dead_code)]
/*
347. 前 K 个高频元素
中等
相关标签
相关企业
给你一个整数数组 nums 和一个整数 k ，请你返回其中出现频率前 k 高的元素。你可以按 任意顺序 返回答案。



示例 1:

输入: nums = [1,1,1,2,2,3], k = 2
输出: [1,2]
示例 2:

输入: nums = [1], k = 1
输出: [1]


提示：

1 <= nums.length <= 105
k 的取值范围是 [1, 数组中不相同的元素的个数]
题目数据保证答案唯一，换句话说，数组中前 k 个高频元素的集合是唯一的


进阶：你所设计算法的时间复杂度 必须 优于 O(n log n) ，其中 n 是数组大小。
 */
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Solution;

/*
top-k 问题  => 使用堆（优先队列）解决
 */
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut cnt_map = HashMap::new();
        nums.into_iter().for_each(|x| {
            cnt_map.entry(x).and_modify(|x| *x += 1).or_insert(1);
        });

        let mut heap = BinaryHeap::with_capacity(k as usize);
        heap.extend(
            cnt_map.iter()
                .take(k as usize)
                .map(|x| (Reverse(*x.1), *x.0))
        );
        for (&val, &cnt) in cnt_map.iter().skip(k as usize) {
            if cnt > heap.peek().unwrap().0.0 {
                heap.pop();
                heap.push((Reverse(cnt), val));
            }
        }

        heap.into_sorted_vec().into_iter().map(|x| x.1).collect()
    }
}