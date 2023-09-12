package io.github.snow.greed;

/*
630. 课程表 III
提示
困难
452
相关企业
这里有 n 门不同的在线课程，按从 1 到 n 编号。给你一个数组 courses ，
其中 courses[i] = [durationi, lastDayi] 表示第 i 门课将会 持续 上 durationi 天课，并且必须在不晚于 lastDayi 的时候完成。

你的学期从第 1 天开始。且不能同时修读两门及两门以上的课程。

返回你最多可以修读的课程数目。



示例 1：

输入：courses = [[100, 200], [200, 1300], [1000, 1250], [2000, 3200]]
输出：3
解释：
这里一共有 4 门课程，但是你最多可以修 3 门：
首先，修第 1 门课，耗费 100 天，在第 100 天完成，在第 101 天开始下门课。
第二，修第 3 门课，耗费 1000 天，在第 1100 天完成，在第 1101 天开始下门课程。
第三，修第 2 门课，耗时 200 天，在第 1300 天完成。
第 4 门课现在不能修，因为将会在第 3300 天完成它，这已经超出了关闭日期。
示例 2：

输入：courses = [[1,2]]
输出：1
示例 3：

输入：courses = [[3,2],[4,3]]
输出：0


提示:

1 <= courses.length <= 104
1 <= durationi, lastDayi <= 104
 */

import org.assertj.core.api.WithAssertions;
import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.Comparator;
import java.util.PriorityQueue;

/**
 * 630. 课程表 III
 *
 * @author snow
 * @since 2023/9/11
 */
public class ScheduleCourse implements WithAssertions {
    static class Solution {
        public int scheduleCourse(int[][] courses) {
            Arrays.sort(courses, Comparator.comparingInt(x -> x[1]));
            // 优先队列 存储持续时间
            PriorityQueue<Integer> queue = new PriorityQueue<>(Comparator.reverseOrder());
            // 当前时间
            int day = 0;
            for (int[] x : courses) {
                int time = x[0];
                int limit = x[1];
                if (day + time <= limit) {
                    // 将该课程加入到队列中
                    queue.add(time);
                    day += time;
                } else {
                    // 比较优先队列中最大值
                    if (!queue.isEmpty() && time < queue.peek()) {
                        int pre = queue.poll();
                        queue.add(time);
                        day += time - pre;
                    }
                }
            }
            return queue.size();
        }
    }

    private static final Solution s = new Solution();

    @Test
    public void fun1() {
        int[][] arr = {{100, 200}, {200, 1300}, {1000, 1250}, {2000, 3200}};
        int ans = s.scheduleCourse(arr);
        assertThat(ans).isEqualTo(3);
    }
}
