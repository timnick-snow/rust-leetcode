package io.github.snow.tree;


/*
2603. 收集树中金币
困难
相关标签
相关企业
提示
给你一个 n 个节点的无向无根树，节点编号从 0 到 n - 1 。给你整数 n 和一个长度为 n - 1 的二维整数数组 edges ，其中 edges[i] = [ai, bi] 表示树中节点 ai 和 bi 之间有一条边。再给你一个长度为 n 的数组 coins ，其中 coins[i] 可能为 0 也可能为 1 ，1 表示节点 i 处有一个金币。

一开始，你需要选择树中任意一个节点出发。你可以执行下述操作任意次：

收集距离当前节点距离为 2 以内的所有金币，或者
移动到树中一个相邻节点。
你需要收集树中所有的金币，并且回到出发节点，请你返回最少经过的边数。

如果你多次经过一条边，每一次经过都会给答案加一。



示例 1：



输入：coins = [1,0,0,0,0,1], edges = [[0,1],[1,2],[2,3],[3,4],[4,5]]
输出：2
解释：从节点 2 出发，收集节点 0 处的金币，移动到节点 3 ，收集节点 5 处的金币，然后移动回节点 2 。
示例 2：



输入：coins = [0,0,0,1,1,0,0,1], edges = [[0,1],[0,2],[1,3],[1,4],[2,5],[5,6],[5,7]]
输出：2
解释：从节点 0 出发，收集节点 4 和 3 处的金币，移动到节点 2 处，收集节点 7 处的金币，移动回节点 0 。


提示：

n == coins.length
1 <= n <= 3 * 104
0 <= coins[i] <= 1
edges.length == n - 1
edges[i].length == 2
0 <= ai, bi < n
ai != bi
edges 表示一棵合法的树。
 */

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;

/**
 * 2603. 收集树中金币
 *
 * @author snow
 * @since 2023/9/21
 */
public class CollectTheCoins {
    /*
     * 无根无向树： n个节点共有(n-1)条边 树不会交叉
     *
     * 节点的度：经过该节点的边的数量，度为1表示该节点是叶子节点
     *
     * 首先去除不含有金币的叶子节点，我们不需要去这些节点
     */
    static class Solution {
        public int collectTheCoins(int[] coins, int[][] edges) {
            int n = coins.length;
            List<Integer>[] g = new List[n];
            for (int i = 0; i < n; i++) {
                g[i] = new ArrayList<>();
            }
            int[] degree = new int[n];
            for (int[] edge : edges) {
                int a = edge[0], b = edge[1];
                g[a].add(b);
                g[b].add(a);
                degree[a]++;
                degree[b]++;
            }
            // 去除没有金币的叶子节点
            ArrayDeque<Integer> queue = new ArrayDeque<>();
            for (int i = 0; i < n; i++) {
                if (degree[i] == 1 && coins[i] == 0) {
                    queue.addLast(i);
                }
            }
            int rest = n;
            while (!queue.isEmpty()) {
                int x = queue.pollFirst();
                rest--;
                degree[x]--;
                for (int y : g[x]) {
                    degree[y]--;
                    if (degree[y] == 1 && coins[y] == 0) {
                        queue.addLast(y);
                    }
                }
            }
            // 删除所有的叶子节点  连续删除2次
            for (int i = 0; i < 2; i++) {
                for (int j = 0; j < n; j++) {
                    if (degree[j] == 1) {
                        queue.addLast(j);
                    }
                }
                while (!queue.isEmpty()) {
                    int x = queue.pollFirst();
                    rest--;
                    degree[x]--;
                    for (int y : g[x]) {
                        degree[y]--;
                    }
                }
            }
            return rest == 0 ? 0 : (rest - 1) * 2;
        }
    }
}
