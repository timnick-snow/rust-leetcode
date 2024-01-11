package io.github.snow.str;

/*
2645. 构造有效字符串的最少插入数
已解答
中等
相关标签
相关企业
提示
给你一个字符串 word ，你可以向其中任何位置插入 "a"、"b" 或 "c" 任意次，返回使 word 有效 需要插入的最少字母数。

如果字符串可以由 "abc" 串联多次得到，则认为该字符串 有效 。



示例 1：

输入：word = "b"
输出：2
解释：在 "b" 之前插入 "a" ，在 "b" 之后插入 "c" 可以得到有效字符串 "abc" 。
示例 2：

输入：word = "aaa"
输出：6
解释：在每个 "a" 之后依次插入 "b" 和 "c" 可以得到有效字符串 "abcabcabc" 。
示例 3：

输入：word = "abc"
输出：0
解释：word 已经是有效字符串，不需要进行修改。


提示：

1 <= word.length <= 50
word 仅由字母 "a"、"b" 和 "c" 组成。
 */

/**
 * @author snow
 * @since 2024/1/11
 */
public class AddMinimum {
    static class Solution {
        public int addMinimum(String word) {
            int ans = 0;
            int flag = -1;
            for (int i = 0; i < word.length(); i++) {
                // 0 1 2
                int c = word.charAt(i) - 'a';
                if (c - flag == 1) {
                    flag = c == 2 ? -1 : c;
                    continue;
                }
                if (c <= flag) {
                    if (flag != -1) {
                        ans += 2 - flag;
                    }
                    ans += c;
                } else {
                    ans += c - (flag + 1);
                }

                flag = c == 2 ? -1 : c;
                // bb
                // -1 1 2  xb xc
                // 0  0 2  aa ac
                // 1 0 1   aba abb
            }
            if (flag != -1) {
                ans += 2 - flag;
            }
            return ans;
        }
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        System.out.println(solution.addMinimum("b"));
        System.out.println(solution.addMinimum("c"));
        System.out.println(solution.addMinimum("aa"));
        System.out.println(solution.addMinimum("ac"));
        System.out.println(solution.addMinimum("aba"));
        System.out.println(solution.addMinimum("abb"));
    }
}
