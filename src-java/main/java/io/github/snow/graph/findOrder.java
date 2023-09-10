package io.github.snow.graph;

/*
210. 课程表 II
提示
中等
823
相关企业
现在你总共有 numCourses 门课需要选，记为 0 到 numCourses - 1。给你一个数组 prerequisites ，
其中 prerequisites[i] = [ai, bi] ，表示在选修课程 ai 前 必须 先选修 bi 。

例如，想要学习课程 0 ，你需要先完成课程 1 ，我们用一个匹配来表示：[0,1] 。
返回你为了学完所有课程所安排的学习顺序。可能会有多个正确的顺序，你只要返回 任意一种 就可以了。如果不可能完成所有课程，返回 一个空数组 。



示例 1：

输入：numCourses = 2, prerequisites = [[1,0]]
输出：[0,1]
解释：总共有 2 门课程。要学习课程 1，你需要先完成课程 0。因此，正确的课程顺序为 [0,1] 。
示例 2：

输入：numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
输出：[0,2,1,3]
解释：总共有 4 门课程。要学习课程 3，你应该先完成课程 1 和课程 2。并且课程 1 和课程 2 都应该排在课程 0 之后。
因此，一个正确的课程顺序是 [0,1,2,3] 。另一个正确的排序是 [0,2,1,3] 。
示例 3：

输入：numCourses = 1, prerequisites = []
输出：[0]


提示：
1 <= numCourses <= 2000
0 <= prerequisites.length <= numCourses * (numCourses - 1)
prerequisites[i].length == 2
0 <= ai, bi < numCourses
ai != bi
所有[ai, bi] 互不相同 */

import org.assertj.core.api.WithAssertions;
import org.junit.jupiter.api.Test;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * 210. 课程表 II
 *
 * @author snow
 * @since 2023/9/10
 */
public class findOrder implements WithAssertions {
    /*
     * 队列解法
     */
    static class Solution {
        public int[] findOrder(int numCourses, int[][] prerequisites) {
            // u -> v 学习了u才可以学习v
            List<List<Integer>> edges = new ArrayList<>();
            for (int i = 0; i < numCourses; i++) {
                edges.add(new ArrayList<>());
            }
            int[] indeg = new int[numCourses];
            // 初始化边和入度
            for (int[] prerequisite : prerequisites) {
                int a = prerequisite[0], b = prerequisite[1];
                edges.get(b).add(a);
                indeg[a]++;
            }
            // 开始拓扑排序
            int[] ans = new int[numCourses];
            ArrayDeque<Integer> deque = new ArrayDeque<>();
            for (int i = 0; i < numCourses; i++) {
                if (indeg[i] == 0) {
                    deque.addLast(i);
                }
            }
            int i = 0;
            while (!deque.isEmpty()) {
                int n = deque.pollFirst();
                ans[i] = n;
                for (int x : edges.get(n)) {
                    indeg[x]--;
                    if (indeg[x] == 0) {
                        deque.addLast(x);
                    }
                }
                i++;
            }
            if (i == numCourses) {
                return ans;
            } else {
                return new int[0];
            }
        }
    }

    private static final Solution solution = new Solution();

    @Test
    public void fun1() throws Exception {
        int[][] arr = {{1, 0}};
        int[] order = solution.findOrder(2, arr);
        System.out.println(Arrays.toString(order));
    }
}
