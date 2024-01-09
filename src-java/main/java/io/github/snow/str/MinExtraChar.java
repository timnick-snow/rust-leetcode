package io.github.snow.str;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Set;

/*
2707. 字符串中的额外字符
已解答
中等
相关标签
相关企业
提示
给你一个下标从 0 开始的字符串 s 和一个单词字典 dictionary 。你需要将 s 分割成若干个 互不重叠 的子字符串，每个子字符串都在 dictionary 中出现过。s 中可能会有一些 额外的字符 不在任何子字符串中。

请你采取最优策略分割 s ，使剩下的字符 最少 。



示例 1：

输入：s = "leetscode", dictionary = ["leet","code","leetcode"]
输出：1
解释：将 s 分成两个子字符串：下标从 0 到 3 的 "leet" 和下标从 5 到 8 的 "code" 。只有 1 个字符没有使用（下标为 4），所以我们返回 1 。
示例 2：

输入：s = "sayhelloworld", dictionary = ["hello","world"]
输出：3
解释：将 s 分成两个子字符串：下标从 3 到 7 的 "hello" 和下标从 8 到 12 的 "world" 。下标为 0 ，1 和 2 的字符没有使用，所以我们返回 3 。


提示：

1 <= s.length <= 50
1 <= dictionary.length <= 50
1 <= dictionary[i].length <= 50
dictionary[i] 和 s 只包含小写英文字母。
dictionary 中的单词互不相同。
 */

/**
 * @author snow
 * @since 2024/1/9
 */
public class MinExtraChar {
    static class Solution {
        // 动态规划
        public int minExtraChar(String s, String[] dictionary) {
            int n = s.length();
            HashMap<Character, Set<String>> map = new HashMap<>();
            for (String word : dictionary) {
                char key = word.charAt(word.length() - 1);
                map.computeIfAbsent(key, HashSet::new);
                // 以最后一个字符作为key分组word
                Set<String> set = map.get(key);
                set.add(word);
            }

            // dp[i] s中的前i个字符作为字符串时的剩余数
            int[] dp = new int[n + 1];
            for (int i = 0; i < n; i++) {
                char key = s.charAt(i);
                dp[i + 1] = dp[i] + 1;
                if (!map.containsKey(key)) {
                    continue;
                }

                String segment = s.substring(0, i + 1);
                Set<String> set = map.get(key);
                for (String word : set) {
                    if (!segment.endsWith(word)) {
                        continue;
                    }
                    dp[i + 1] = Math.min(dp[i + 1], dp[i + 1 - word.length()]);
                }
            }

            return dp[n];
        }

        // 暴力 - 超时
        public int minExtraChar2(String s, String[] dictionary) {
            int res = s.length();
            for (String word : dictionary) {
                int idx = s.indexOf(word);
                if (idx == -1) {
                    continue;
                }
                String rest = s.substring(idx + word.length());
                res = Math.min(res, minExtraChar(rest, dictionary) + idx);
            }

            return res;
        }
    }
}
