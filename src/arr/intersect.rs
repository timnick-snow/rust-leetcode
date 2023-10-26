#![allow(dead_code)]
/*
350. 两个数组的交集 II
简单
相关标签
相关企业
给你两个整数数组 nums1 和 nums2 ，请你以数组形式返回两数组的交集。返回结果中每个元素出现的次数，应与元素在两个数组中都出现的次数一致（如果出现次数不一致，则考虑取较小值）。可以不考虑输出结果的顺序。



示例 1：

输入：nums1 = [1,2,2,1], nums2 = [2,2]
输出：[2,2]
示例 2:

输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出：[4,9]


提示：

1 <= nums1.length, nums2.length <= 1000
0 <= nums1[i], nums2[i] <= 1000


进阶：

如果给定的数组已经排好序呢？你将如何优化你的算法？
如果 nums1 的大小比 nums2 小，哪种方法更优？
如果 nums2 的元素存储在磁盘上，内存是有限的，并且你不能一次加载所有的元素到内存中，你该怎么办？
 */
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() > nums2.len() {
            let temp = nums1;
            nums1 = nums2;
            nums2 = temp;
        }
        let mut map = HashMap::new();
        for x in nums1.into_iter() {
            map.entry(x).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut ans = Vec::with_capacity(map.len());
        for x in nums2.into_iter() {
            if map.contains_key(&x) {
                ans.push(x);
                map.entry(x).and_modify(|x| *x -= 1);
                if *map.get(&x).unwrap() == 0 {
                    map.remove(&x);
                }
            }
        }
        ans
    }
}