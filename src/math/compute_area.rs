#![allow(dead_code)]
/*
223. 矩形面积
中等
239
相关企业
给你 二维 平面上两个 由直线构成且边与坐标轴平行/垂直 的矩形，请你计算并返回两个矩形覆盖的总面积。

每个矩形由其 左下 顶点和 右上 顶点坐标表示：

第一个矩形由其左下顶点 (ax1, ay1) 和右上顶点 (ax2, ay2) 定义。
第二个矩形由其左下顶点 (bx1, by1) 和右上顶点 (bx2, by2) 定义。


示例 1：

Rectangle Area
输入：ax1 = -3, ay1 = 0, ax2 = 3, ay2 = 4, bx1 = 0, by1 = -1, bx2 = 9, by2 = 2
输出：45
示例 2：

输入：ax1 = -2, ay1 = -2, ax2 = 2, ay2 = 2, bx1 = -2, by1 = -2, bx2 = 2, by2 = 2
输出：16


提示：

-104 <= ax1, ay1, ax2, ay2, bx1, by1, bx2, by2 <= 104
 */
struct Solution;

/*
面积之和减去重叠面积即可
 */
impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let overlap_w = Self::helper(ax1, ax2, bx1, bx2);
        let overlap_h = Self::helper(ay1, ay2, by1, by2);
        (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1) - overlap_w * overlap_h
    }

    fn helper(a1: i32, mut a2: i32, mut b1: i32, mut b2: i32) -> i32 {
        if a1 > b1 {
            let (t1, t2) = (a1, a2);
            a2 = b2;
            b1 = t1;
            b2 = t2;
        }
        if b1 >= a2 {
            0
        } else {
            std::cmp::min(a2, b2) - b1
        }
    }
}