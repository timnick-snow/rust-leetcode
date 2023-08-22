#![allow(dead_code)]
/*
849. 到最近的人的最大距离
中等
216
相关企业
给你一个数组 seats 表示一排座位，其中 seats[i] = 1 代表有人坐在第 i 个座位上，seats[i] = 0 代表座位 i 上是空的（下标从 0 开始）。

至少有一个空座位，且至少有一人已经坐在座位上。

亚历克斯希望坐在一个能够使他与离他最近的人之间的距离达到最大化的座位上。

返回他到离他最近的人的最大距离。



示例 1：


输入：seats = [1,0,0,0,1,0,1]
输出：2
解释：
如果亚历克斯坐在第二个空位（seats[2]）上，他到离他最近的人的距离为 2 。
如果亚历克斯坐在其它任何一个空位上，他到离他最近的人的距离为 1 。
因此，他到离他最近的人的最大距离是 2 。
示例 2：

输入：seats = [1,0,0,0]
输出：3
解释：
如果亚历克斯坐在最后一个座位上，他离最近的人有 3 个座位远。
这是可能的最大距离，所以答案是 3 。
示例 3：

输入：seats = [0,1]
输出：1


提示：

2 <= seats.length <= 2 * 104
seats[i] 为 0 或 1
至少有一个 空座位
至少有一个 座位上有人
 */
struct Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let (mut left_most, mut right_most) = (0, seats.len() - 1);
        let mut ans = 1;
        // 前置0处理
        while left_most <= right_most && seats[left_most] == 0 {
            left_most += 1;
        }
        ans = std::cmp::max(ans, left_most);
        // 后置0处理
        while right_most >= left_most && seats[right_most] == 0 {
            right_most -= 1;
        }
        ans = std::cmp::max(ans, seats.len() - 1 - right_most);
        // 处理夹在中间的空位
        let mut i = left_most + 1;
        while i <= right_most {
            if seats[i] == 1 {
                ans = std::cmp::max((i - left_most) >> 1, ans);
                left_most = i;
            }
            i += 1;
        }
        ans as i32
    }
}

#[cfg(test)]
mod test {
    use crate::arr::max_dist_to_closest::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::max_dist_to_closest(vec![1, 0, 0, 0]);
        println!("{}", ans);
    }
    #[test]
    pub fn t2() {
        // [1,1,0,0,0,1,0]
        let ans = Solution::max_dist_to_closest(vec![1,1,0,0,0,1,0]);
        println!("{}", ans);
    }
}