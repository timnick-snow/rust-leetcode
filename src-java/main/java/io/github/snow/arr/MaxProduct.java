package io.github.snow.arr;
/*
318. 最大单词长度乘积
中等
相关标签
相关企业
给你一个字符串数组 words ，找出并返回 length(words[i]) * length(words[j]) 的最大值，并且这两个单词不含有公共字母。如果不存在这样的两个单词，返回 0 。



示例 1：

输入：words = ["abcw","baz","foo","bar","xtfn","abcdef"]
输出：16
解释：这两个单词为 "abcw", "xtfn"。
示例 2：

输入：words = ["a","ab","abc","d","cd","bcd","abcd"]
输出：4
解释：这两个单词为 "ab", "cd"。
示例 3：

输入：words = ["a","aa","aaa","aaaa"]
输出：0
解释：不存在这样的两个单词。


提示：

2 <= words.length <= 1000
1 <= words[i].length <= 1000
words[i] 仅包含小写字母
 */

import java.util.HashMap;
import java.util.Map;
import java.util.Set;

/**
 * 318. 最大单词长度乘积
 *
 * @author snow
 * @since 2023/11/6
 */
public class MaxProduct {
    /*
     * 由于单词只包含小写字母（26种），用int类型来保存种类
     */
    static class Solution {
        static final int ALL_KIND = 0x11ff_ffff;

        public int maxProduct(String[] words) {
            Map<Integer, Integer> map = new HashMap<>(16);
            for (String word : words) {
                int kind = getKind(word);
                if (word.length() > map.getOrDefault(kind, 0)) {
                    map.put(kind, word.length());
                }
            }
            Set<Integer> set = map.keySet();
            int ans = 0;
            for (Integer k1 : set) {
                for (Integer k2 : set) {
                    if ((k1&k2)==0) {
                        // k1,k2不含有相同字母
                        ans = Math.max(ans, map.get(k1) * map.get(k2));
                    }
                }
            }
            return ans;
        }

        private int getKind(String word) {
            int bit = 0;
            for (int i = 0; i < word.length(); i++) {
                bit |= 1 << (word.charAt(i) - 'a');
                if ((bit ^ ALL_KIND) == 0) {
                    return ALL_KIND;
                }
            }
            return bit;
        }
    }
}
