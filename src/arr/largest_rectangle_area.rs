#![allow(dead_code)]

/*
84. 柱状图中最大的矩形
困难
2.5K
相关企业
给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。

求在该柱状图中，能够勾勒出来的矩形的最大面积。



示例 1:



输入：heights = [2,1,5,6,2,3]
输出：10
解释：最大的矩形为图中红色区域，面积为 10
示例 2：



输入： heights = [2,4]
输出： 4


提示：

1 <= heights.length <=105
0 <= heights[i] <= 104
 */
struct Solution;
/*
对于每一个柱子i，我们试图以该柱子的高度heights[i]进行勾勒出一个最大的矩形。
这样，当我们枚举完所有柱子[i..n-1]，必然将包含能够勾勒出的最大矩形。

这是因为，矩形的最大面积必然是以其覆盖的所有柱子的最低高度为高，柱子的数量为长。
而我们枚举了每一个柱子的高度所能勾勒的最大面积，也就包含了这个最大矩形。

那么问题变成：对于每一个柱子，我们要找到左右两边最近的低于此柱子高度的位置
直接使用暴力方法找的话，复杂度过高

单调栈：使用单调栈可以方便的找到最近的低于柱子高度的位置
我们从左到右、从右到左分别扫描一次，就找到两边的位置了。

最后进行求解
 */
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        if n == 0 { return 0; }
        if n == 1 { return heights[0]; }

        let mut vec = Vec::new();
        let mut left = vec![0; n];
        let mut right = vec![0; n];

        // 从左到右扫描
        for i in 0..n {
            while !vec.is_empty() && heights[i] <= heights[*vec.last().unwrap()] {
                vec.pop();
            }
            // while let Some(&x) = vec.last() && heights[i] <= heights[x] {
            //     vec.pop();
            // }
            left[i] = vec.last().map_or(-1, |x| *x as i32);
            vec.push(i);
        }
        vec.clear();
        // 从右到左扫描
        for i in (0..n).rev() {
            while !vec.is_empty() && heights[i] <= heights[*vec.last().unwrap()] {
                vec.pop();
            }
            right[i] = vec.last().map_or(n as i32, |x| *x as i32);
            vec.push(i);
        }
        // 求解最大面积
        left.iter().zip(right.iter())
            .enumerate()
            .map(|(i, (x, y))| heights[i] * (y - x - 1))
            .max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::arr::largest_rectangle_area::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]);
        assert_eq!(ans, 10);
    }


    #[test]
    pub fn t2() {
        let ans = Solution::largest_rectangle_area(vec![0, 9]);
        assert_eq!(ans, 9);
    }

    #[test]
    pub fn t3() {
        let ans = Solution::largest_rectangle_area(vec![4, 2]);
        assert_eq!(ans, 4);
    }
}