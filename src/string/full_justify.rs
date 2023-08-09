#![allow(dead_code)]

/*
68. 文本左右对齐
困难
359
相关企业
给定一个单词数组 words 和一个长度 maxWidth ，重新排版单词，使其成为每行恰好有 maxWidth 个字符，且左右两端对齐的文本。

你应该使用 “贪心算法” 来放置给定的单词；也就是说，尽可能多地往每行中放置单词。必要时可用空格 ' ' 填充，使得每行恰好有 maxWidth 个字符。

要求尽可能均匀分配单词间的空格数量。如果某一行单词间的空格不能均匀分配，则左侧放置的空格数要多于右侧的空格数。

文本的最后一行应为左对齐，且单词之间不插入额外的空格。

注意:

单词是指由非空格字符组成的字符序列。
每个单词的长度大于 0，小于等于 maxWidth。
输入单词数组 words 至少包含一个单词。


示例 1:

输入: words = ["This", "is", "an", "example", "of", "text", "justification."], maxWidth = 16
输出:
[
   "This    is    an",
   "example  of text",
   "justification.  "
]
示例 2:

输入:words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
输出:
[
  "What   must   be",
  "acknowledgment  ",
  "shall be        "
]
解释: 注意最后一行的格式应为 "shall be    " 而不是 "shall     be",
     因为最后一行应为左对齐，而不是左右两端对齐。
     第二行同样为左对齐，这是因为这行只包含一个单词。
示例 3:

输入:words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"]，maxWidth = 20
输出:
[
  "Science  is  what we",
  "understand      well",
  "enough to explain to",
  "a  computer.  Art is",
  "everything  else  we",
  "do                  "
]


提示:

1 <= words.length <= 300
1 <= words[i].length <= 20
words[i] 由小写英文字母和符号组成
1 <= maxWidth <= 100
words[i].length <= maxWidth
 */
struct Solution;

use std::iter;


#[derive(Debug)]
struct Line {
    max_width: usize,
    cur_width: usize,
    data: Vec<String>,
}

impl Line {
    fn new(max_width: usize) -> Self {
        Line {
            max_width,
            cur_width: 0,
            data: Vec::new(),
        }
    }
    fn try_add(&mut self, word: String) -> Result<(), ()> {
        if self.cur_width + self.data.len() + word.len() > self.max_width {
            Err(())
        } else {
            self.cur_width += word.len();
            self.data.push(word);
            Ok(())
        }
    }
    fn justify(self, last_line: bool) -> String {
        if last_line {
            // 最后一行左对齐 不加额外空格
            let line_str = self.data.into_iter()
                .reduce(|mut line, word| {
                    line.push(' ');
                    line.push_str(&word);
                    line
                }).unwrap();
            format!("{:width$}", line_str, width = self.max_width)
        } else {
            if self.data.len() == 1 {
                return format!("{:width$}", self.data[0], width = self.max_width);
            }
            // 计算空格分配
            let space_len = self.max_width - self.cur_width;
            let min = space_len / (self.data.len() - 1);
            let more_count = space_len % (self.data.len() - 1);
            // 空格迭代器
            let space_iter =
                iter::repeat(iter::repeat(' ').take(min + 1).collect::<String>())
                    .take(more_count)
                    .chain(
                        iter::repeat(iter::repeat(' ').take(min).collect::<String>())
                            .take(self.data.len() - 1 - more_count)
                    )
                    .chain(iter::once("".to_string()));
            self.data.into_iter()
                .zip(space_iter)
                .fold(String::new(), |mut line_str, (s1, s2)| {
                    line_str.push_str(&s1);
                    line_str.push_str(&s2);
                    line_str
                })
        }
    }
}


impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut ans = Vec::new();
        let mut line = Line::new(max_width);
        for word in words.into_iter() {
            if line.try_add(word.clone()).is_err() {
                // 添加该行到结果集
                ans.push(line.justify(false));
                // 重新构造line
                line = Line::new(max_width);
                line.try_add(word).expect("will not panic");
            }
        }
        ans.push(line.justify(true));
        ans
    }
}