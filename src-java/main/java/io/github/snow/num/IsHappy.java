package io.github.snow.num;
/*
202. 快乐数
简单
1.4K
相关企业
编写一个算法来判断一个数 n 是不是快乐数。

「快乐数」 定义为：

对于一个正整数，每一次将该数替换为它每个位置上的数字的平方和。
然后重复这个过程直到这个数变为 1，也可能是 无限循环 但始终变不到 1。
如果这个过程 结果为 1，那么这个数就是快乐数。
如果 n 是 快乐数 就返回 true ；不是，则返回 false 。



示例 1：

输入：n = 19
输出：true
解释：
12 + 92 = 82
82 + 22 = 68
62 + 82 = 100
12 + 02 + 02 = 1
示例 2：

输入：n = 2
输出：false


提示：

1 <= n <= 231 - 1
 */

import org.assertj.core.api.WithAssertions;
import org.junit.jupiter.api.Test;

import java.util.HashSet;

/**
 * 202. 快乐数
 *
 * @author snow
 * @since 2023/9/6
 */
public class IsHappy implements WithAssertions {
    static class Solution {
        public boolean isHappy(int n) {
            HashSet<Integer> set = new HashSet<>();
            while (n != 1) {
                if (!set.add(n)) {
                    return false;
                }
                n = powSum(n);
            }
            return true;
        }

        private int powSum(int x) {
            int sum = 0;
            while (x != 0) {
                int y = x % 10;
                sum += y * y;
                x = x / 10;
            }
            return sum;
        }
    }

    private static final Solution solution = new Solution();

    @Test
    public void fun1() throws Exception {
        assertThat(solution.isHappy(19)).isTrue();
        assertThat(solution.isHappy(2)).isTrue();
    }
}
