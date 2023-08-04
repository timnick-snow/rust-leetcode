#![allow(dead_code)]


/*
42. 接雨水
困难
4.6K
相关企业
给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。



示例 1：



输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
输出：6
解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。
示例 2：

输入：height = [4,2,0,3,2,5]
输出：9


提示：

n == height.length
1 <= n <= 2 * 104
0 <= height[i] <= 105

单调栈
 */
struct Solution;

use std::cmp::min;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut ans = 0;
        for (i, x) in height.into_iter().enumerate() {
            if stack.is_empty() {
                stack.push((i, x));
            } else {
                if stack.last().unwrap().clone().1 >= x {
                    // 单调递减 直接入栈
                    stack.push((i, x));
                } else {
                    // 高度开始上升
                    let mut pre = 0;
                    while let Some(&(index, h)) = stack.last() {
                        if h <= x {
                            stack.pop();
                            ans += (i - index - 1) as i32 * (min(h, x) - pre);
                            pre = h;
                            // println!("{:?} <==> 水位增加至{}, pre={}", (i, index), ans, pre);
                        } else {
                            ans += (i - index - 1) as i32 * (x - pre);
                            // println!("{:?} <==> 水位增加至{}, pre={}", (i, index), ans, pre);
                            break;
                        }
                    }
                    stack.push((i, x));
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::dynamic::trap::Solution;

    #[test]
    fn t1() {
        let ans = Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(ans, 6)
    }

    #[test]
    fn t2() {
        /*
                                                |
         |                                      |
         |                      |               |
         |      |               |       |       |
         |      |       _       |       |       |
         0      1       2       3       4       5
         */
        let ans = Solution::trap(vec![4, 2, 0, 3, 2, 5]);
        assert_eq!(ans, 9)
    }
}