#![allow(dead_code)]
/*
228. 汇总区间
简单
298
相关企业
给定一个  无重复元素 的 有序 整数数组 nums 。

返回 恰好覆盖数组中所有数字 的 最小有序 区间范围列表 。也就是说，nums 的每个元素都恰好被某个区间范围所覆盖，并且不存在属于某个范围但不属于 nums 的数字 x 。

列表中的每个区间范围 [a,b] 应该按如下格式输出：

"a->b" ，如果 a != b
"a" ，如果 a == b


示例 1：

输入：nums = [0,1,2,4,5,7]
输出：["0->2","4->5","7"]
解释：区间范围是：
[0,2] --> "0->2"
[4,5] --> "4->5"
[7,7] --> "7"
示例 2：

输入：nums = [0,2,3,4,6,8,9]
输出：["0","2->4","6","8->9"]
解释：区间范围是：
[0,0] --> "0"
[2,4] --> "2->4"
[6,6] --> "6"
[8,9] --> "8->9"


提示：

0 <= nums.length <= 20
-231 <= nums[i] <= 231 - 1
nums 中的所有值都 互不相同
nums 按升序排列
 */
struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ans = Vec::new();
        let mut start = 0;
        while start < nums.len() {
            let mut end = start;
            while end + 1 < nums.len() {
                if nums[end + 1] - nums[end] != 1 {
                    break;
                }
                end += 1;
            }
            let range = if end == start {
                nums[start].to_string()
            } else {
                let mut temp = String::new();
                temp.push_str(&nums[start].to_string());
                temp.push_str("->");
                temp.push_str(&nums[end].to_string());
                temp
            };
            ans.push(range);
            start = end + 1;
        }
        ans
    }
}