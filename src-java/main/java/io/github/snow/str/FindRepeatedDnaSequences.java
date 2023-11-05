package io.github.snow.str;
/*
187. 重复的DNA序列
已解答
中等
相关标签
相关企业
DNA序列 由一系列核苷酸组成，缩写为 'A', 'C', 'G' 和 'T'.。

例如，"ACGAATTCCG" 是一个 DNA序列 。
在研究 DNA 时，识别 DNA 中的重复序列非常有用。

给定一个表示 DNA序列 的字符串 s ，返回所有在 DNA 分子中出现不止一次的 长度为 10 的序列(子字符串)。你可以按 任意顺序 返回答案。



示例 1：

输入：s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
输出：["AAAAACCCCC","CCCCCAAAAA"]
示例 2：

输入：s = "AAAAAAAAAAAAA"
输出：["AAAAAAAAAA"]


提示：

0 <= s.length <= 105
s[i]=='A'、'C'、'G' or 'T'
 */

import java.util.*;

/**
 * 187. 重复的DNA序列
 *
 * @author snow
 * @since 2023/11/5
 */
public class FindRepeatedDnaSequences {
    static class Solution {
        Map<Character, Integer> map;

        public Solution() {
            this.map = new HashMap<>();
            map.put('A', 0b00);
            map.put('C', 0b01);
            map.put('G', 0b10);
            map.put('T', 0b11);
        }

        public List<String> findRepeatedDnaSequences(String s) {
            if (s.length() < 10) {
                return Collections.emptyList();
            }
            Set<Integer> set = new HashSet<>();
            int value = 0;
            for (int i = 0; i < 10; i++) {
                value <<= 2;
                value |= map.get(s.charAt(i));
            }
            set.add(value);

            Set<String> ans = new HashSet<>();
            for (int i = 10; i < s.length(); i++) {
                value = ((value << 2) | map.get(s.charAt(i))) & ((1 << 20) - 1);
                if (!set.add(value)) {
                    ans.add(s.substring(i - 9, i + 1));
                }
            }
            return new ArrayList<>(ans);
        }
    }
}
