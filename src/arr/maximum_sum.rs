#![allow(dead_code)]
/*
2342. 数位和相等数对的最大和
中等
相关标签
相关企业
提示
给你一个下标从 0 开始的数组 nums ，数组中的元素都是 正 整数。请你选出两个下标 i 和 j（i != j），且 nums[i] 的数位和 与  nums[j] 的数位和相等。

请你找出所有满足条件的下标 i 和 j ，找出并返回 nums[i] + nums[j] 可以得到的 最大值 。



示例 1：

输入：nums = [18,43,36,13,7]
输出：54
解释：满足条件的数对 (i, j) 为：
- (0, 2) ，两个数字的数位和都是 9 ，相加得到 18 + 36 = 54 。
- (1, 4) ，两个数字的数位和都是 7 ，相加得到 43 + 7 = 50 。
所以可以获得的最大和是 54 。
示例 2：

输入：nums = [10,12,19,14]
输出：-1
解释：不存在满足条件的数对，返回 -1 。


提示：

1 <= nums.length <= 105
1 <= nums[i] <= 109
 */
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Solution;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, BinaryHeap<Reverse<i32>>> = HashMap::new();
        nums.into_iter().for_each(|x| {
            let heap = map.entry(digit_sum(x)).or_default();
            if heap.len() < 2 {
                heap.push(Reverse(x));
            } else {
                if x > heap.peek().unwrap().0 {
                    heap.pop();
                    heap.push(Reverse(x));
                }
            }
        });

        let mut ans = -1;
        for mut heap in map.into_values() {
            if heap.len() == 2 {
                let x = heap.pop().unwrap().0;
                let y = heap.pop().unwrap().0;
                ans = ans.max(x + y);
            }
        }

        ans
    }
}

fn digit_sum(mut num: i32) -> i32 {
    let mut res = 0;
    while num != 0 {
        res += num % 10;
        num /= 10;
    }
    res
}