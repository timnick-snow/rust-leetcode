package io.github.snow.arr;
/*
57. 插入区间
中等
730
相关企业
给你一个 无重叠的 ，按照区间起始端点排序的区间列表。

在列表中插入一个新的区间，你需要确保列表中的区间仍然有序且不重叠（如果有必要的话，可以合并区间）。



示例 1：

输入：intervals = [[1,3],[6,9]], newInterval = [2,5]
输出：[[1,5],[6,9]]
示例 2：

输入：intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
输出：[[1,2],[3,10],[12,16]]
解释：这是因为新的区间 [4,8] 与 [3,5],[6,7],[8,10] 重叠。
示例 3：

输入：intervals = [], newInterval = [5,7]
输出：[[5,7]]
示例 4：

输入：intervals = [[1,5]], newInterval = [2,3]
输出：[[1,5]]
示例 5：

输入：intervals = [[1,5]], newInterval = [2,7]
输出：[[1,7]]


提示：

0 <= intervals.length <= 104
intervals[i].length == 2
0 <= intervals[i][0] <= intervals[i][1] <= 105
intervals 根据 intervals[i][0] 按 升序 排列
newInterval.length == 2
0 <= newInterval[0] <= newInterval[1] <= 105
 */

import org.assertj.core.api.WithAssertions;
import org.junit.jupiter.api.Test;

import java.util.Arrays;

/**
 * 57. 插入区间
 *
 * @author snow
 * @since 2023/8/28
 */
public class Insert implements WithAssertions {
    static class Solution {
        public int[][] insert(int[][] intervals, int[] newInterval) {
            int[][] ans = new int[intervals.length + 1][2];
            int cur = 0;
            int left = newInterval[0], right = newInterval[1];
            boolean flag = true;
            for (int[] interval : intervals) {
                if (left > interval[1]) {
                    ans[cur][0] = interval[0];
                    ans[cur][1] = interval[1];
                    cur++;
                } else {
                    if (right < interval[0]) {
                        if (flag) {
                            flag = false;
                            ans[cur][0] = left;
                            ans[cur][1] = right;
                            cur++;
                        }
                        ans[cur][0] = interval[0];
                        ans[cur][1] = interval[1];
                        cur++;
                    } else {
                        left = Math.min(left, interval[0]);
                        right = Math.max(right, interval[1]);
                    }
                }
            }
            if (flag) {
                ans[cur][0] = left;
                ans[cur][1] = right;
                cur++;
            }
            return Arrays.copyOf(ans, cur);
        }
    }

    private final Solution solution = new Solution();

    @Test
    public void fun1() {
        int[][] intervals = {{1, 3}, {6, 9}};
        int[] newInterval = {2, 5};
        int[][] insert = solution.insert(intervals, newInterval);
        for (int[] ints : insert) {
            System.out.println(Arrays.toString(ints));
        }
    }

    @Test
    public void fun2() {
        int[][] intervals = {};
        int[] newInterval = {5, 7};
        int[][] insert = solution.insert(intervals, newInterval);
        for (int[] ints : insert) {
            System.out.println(Arrays.toString(ints));
        }
    }
}

