#![allow(dead_code)]
/*
1726. 同积元组
中等
相关标签
相关企业
提示
给你一个由 不同 正整数组成的数组 nums ，请你返回满足 a * b = c * d 的元组 (a, b, c, d) 的数量。其中 a、b、c 和 d 都是 nums 中的元素，且 a != b != c != d 。



示例 1：

输入：nums = [2,3,4,6]
输出：8
解释：存在 8 个满足题意的元组：
(2,6,3,4) , (2,6,4,3) , (6,2,3,4) , (6,2,4,3)
(3,4,2,6) , (4,3,2,6) , (3,4,6,2) , (4,3,6,2)
示例 2：

输入：nums = [1,2,4,5,10]
输出：16
解释：存在 16 个满足题意的元组：
(1,10,2,5) , (1,10,5,2) , (10,1,2,5) , (10,1,5,2)
(2,5,1,10) , (2,5,10,1) , (5,2,1,10) , (5,2,10,1)
(2,10,4,5) , (2,10,5,4) , (10,2,4,5) , (10,2,4,5)
(4,5,2,10) , (4,5,10,2) , (5,4,2,10) , (5,4,10,2)


提示：

1 <= nums.length <= 1000
1 <= nums[i] <= 104
nums 中的所有元素 互不相同
 */
use std::collections::HashMap;

struct Solution;

/*
直接统计所有二元组的乘积，保存到map中
key: 乘积
value: 次数

那么每一个乘积都可以构成 value*(value-1)*4 元组
 */
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..n {
            for j in i + 1..n {
                let product = nums[i] * nums[j];
                map.entry(product)
                    .and_modify(|cnt| *cnt += 1)
                    .or_insert(1);
            }
        }
        let mut ans = 0;
        for &v in map.values() {
            ans += v * (v - 1) * 4;
        }
        ans
    }
}