
/*
11. 盛最多水的容器
给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i]) 。

找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。

返回容器可以储存的最大水量。

说明：你不能倾斜容器。



示例 1：



输入：[1,8,6,2,5,4,8,3,7]
输出：49
解释：图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
示例 2：

输入：height = [1,1]
输出：1


提示：

n == height.length
2 <= n <= 105
0 <= height[i] <= 104
*/
struct Solution;
use std::cmp::min;


impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max = 0;

        while left < right {
            let area = (right - left) as i32 * min(height[left], height[right]);
            if area > max {
                max = area;
            }
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        };
        max
    }
}

#[cfg(test)]
mod test {
    use crate::greed::water_holder::Solution;

    #[test]
    fn example1() {
        let area = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(area, 49);

        let area = Solution::max_area(vec![1, 1]);
        assert_eq!(area, 1);

        let area = Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]);
        assert_eq!(area, 17);
    }
}