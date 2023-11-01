#![allow(dead_code)]
/*
373. 查找和最小的 K 对数字
中等
相关标签
相关企业
给定两个以 非递减顺序排列 的整数数组 nums1 和 nums2 , 以及一个整数 k 。

定义一对值 (u,v)，其中第一个元素来自 nums1，第二个元素来自 nums2 。

请找到和最小的 k 个数对 (u1,v1),  (u2,v2)  ...  (uk,vk) 。



示例 1:

输入: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
输出: [1,2],[1,4],[1,6]
解释: 返回序列中的前 3 对数：
     [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
示例 2:

输入: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
输出: [1,1],[1,1]
解释: 返回序列中的前 2 对数：
     [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
示例 3:

输入: nums1 = [1,2], nums2 = [3], k = 3
输出: [1,3],[2,3]
解释: 也可能序列中所有的数对都被返回:[1,3],[2,3]


提示:

1 <= nums1.length, nums2.length <= 105
-109 <= nums1[i], nums2[i] <= 109
nums1 和 nums2 均为升序排列
1 <= k <= 104
 */
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

struct Solution;
/*
以下描述的数对均代表原数组中的索引
由于两个数组都已排序，所以最小值的数对必然是(0,0)，下一个值必然是从更小的数对中将其中一个索引加1形成的，即可能是(0,1)和(1,0)
维护下一个数对的可能值，然后进行比较

将数对映射到整数 a*10^5 + b, 方便去重
 */
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let m = nums1.len();
        let n = nums2.len();
        let pair_hash = |i: usize, j: usize| {
            i * n + j
        };
        // 数对
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity((k as usize).min(m * n));
        // 索引对hash
        let mut set: HashSet<usize> = HashSet::new();
        // 最小堆
        let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
        // 初始化堆  最小数对索引一定是(0,0)
        heap.push((Reverse(nums1[0] + nums2[0]), pair_hash(0, 0)));

        // 出堆取出最小值 并将可能的下一个索引对入堆
        while !heap.is_empty() && set.len() < k as usize {
            let (_, hash) = heap.pop().unwrap();
            if set.insert(hash) {
                let (i, j) = (hash / n, hash % n);
                ans.push(vec![nums1[i], nums2[j]]);
                if i + 1 < m {
                    heap.push((Reverse(nums1[i + 1] + nums2[j]), pair_hash(i + 1, j)));
                }
                if j + 1 < n {
                    heap.push((Reverse(nums1[i] + nums2[j + 1]), pair_hash(i, j + 1)));
                }
            }
        }

        ans
    }
}
