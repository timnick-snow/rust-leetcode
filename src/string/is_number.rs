#![allow(dead_code)]
/*
65. 有效数字
困难
358
相关企业
有效数字（按顺序）可以分成以下几个部分：

一个 小数 或者 整数
（可选）一个 'e' 或 'E' ，后面跟着一个 整数
小数（按顺序）可以分成以下几个部分：

（可选）一个符号字符（'+' 或 '-'）
下述格式之一：
至少一位数字，后面跟着一个点 '.'
至少一位数字，后面跟着一个点 '.' ，后面再跟着至少一位数字
一个点 '.' ，后面跟着至少一位数字
整数（按顺序）可以分成以下几个部分：

（可选）一个符号字符（'+' 或 '-'）
至少一位数字
部分有效数字列举如下：["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"]

部分无效数字列举如下：["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]

给你一个字符串 s ，如果 s 是一个 有效数字 ，请返回 true 。



示例 1：

输入：s = "0"
输出：true
示例 2：

输入：s = "e"
输出：false
示例 3：

输入：s = "."
输出：false


提示：

1 <= s.length <= 20
s 仅含英文字母（大写和小写），数字（0-9），加号 '+' ，减号 '-' ，或者点 '.' 。
 */
struct Solution;

enum CharType {
    Number,
    Signal,
    Power,
    Dot,
    Other,
}

trait Token {
    fn judge(&self, ct: CharType, context: &mut Context) -> Option<Box<dyn Token>>;
}

struct Init;

struct Signal1;

struct Num1Int;

struct Num1Dot;

struct Num1Dec;

struct Power;

struct Signal2;

struct Num2;


impl Token for Init {
    fn judge(&self, ct: CharType, context: &mut Context) -> Option<Box<dyn Token>> {
        match ct {
            CharType::Number => {
                context.s1 = true;
                Some(Box::new(Num1Int))
            }
            CharType::Signal => Some(Box::new(Signal1)),
            CharType::Dot => Some(Box::new(Num1Dot)),
            _ => None,
        }
    }
}

impl Token for Signal1 {
    fn judge(&self, ct: CharType, context: &mut Context) -> Option<Box<dyn Token>> {
        match ct {
            CharType::Number => {
                context.s1 = true;
                Some(Box::new(Num1Int))
            }
            CharType::Dot => Some(Box::new(Num1Dot)),
            _ => None,
        }
    }
}

impl Token for Num1Int {
    fn judge(&self, ct: CharType, context: &mut Context) -> Option<Box<dyn Token>> {
        match ct {
            CharType::Number => Some(Box::new(Num1Int)),
            CharType::Dot => Some(Box::new(Num1Dot)),
            CharType::Power => {
                context.s2 = false;
                Some(Box::new(Power))
            }
            _ => None,
        }
    }
}

impl Token for Num1Dot {
    fn judge(&self, ct: CharType, context: &mut Context) -> Option<Box<dyn Token>> {
        match ct {
            CharType::Number => {
                context.s1 = true;
                Some(Box::new(Num1Dec))
            }
            CharType::Power => {
                context.s2 = false;
                Some(Box::new(Power))
            }
            _ => None,
        }
    }
}

impl Token for Num1Dec {
    fn judge(&self, ct: CharType, context: &mut Context) -> Option<Box<dyn Token>> {
        match ct {
            CharType::Number => Some(Box::new(Num1Dec)),
            CharType::Power => {
                context.s2 = false;
                Some(Box::new(Power))
            }
            _ => None,
        }
    }
}

impl Token for Power {
    fn judge(&self, ct: CharType, context: &mut Context) -> Option<Box<dyn Token>> {
        match ct {
            CharType::Number => {
                context.s2 = true;
                Some(Box::new(Num2))
            }
            CharType::Signal => Some(Box::new(Signal2)),
            _ => None,
        }
    }
}

impl Token for Signal2 {
    fn judge(&self, ct: CharType, context: &mut Context) -> Option<Box<dyn Token>> {
        match ct {
            CharType::Number => {
                context.s2 = true;
                Some(Box::new(Num2))
            }
            _ => None,
        }
    }
}

impl Token for Num2 {
    fn judge(&self, ct: CharType, _context: &mut Context) -> Option<Box<dyn Token>> {
        match ct {
            CharType::Number => Some(Box::new(Num2)),
            _ => None,
        }
    }
}

struct Context {
    s1: bool,
    s2: bool,
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut token: Box<dyn Token> = Box::new(Init);
        let context = &mut Context { s1: false, s2: true };
        for x in s.chars() {
            let ct = match x {
                '0'..='9' => CharType::Number,
                '+' | '-' => CharType::Signal,
                'e' | 'E' => CharType::Power,
                '.' => CharType::Dot,
                _ => CharType::Other,
            };
            match token.judge(ct, context) {
                None => {
                    return false;
                }
                Some(next_token) => {
                    token = next_token;
                }
            }
        }
        context.s1 && context.s2
    }
}

#[cfg(test)]
mod test {
    use crate::string::is_number::Solution;

    #[test]
    pub fn t1() {
        // 部分有效数字列举如下：["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"]
        assert!(Solution::is_number("2".into()));
        assert!(Solution::is_number("0089".into()));
        assert!(Solution::is_number("-0.1".into()));
        assert!(Solution::is_number("+3.14".into()));
        assert!(Solution::is_number("4.".into()));
        assert!(Solution::is_number("-.9".into()));
        assert!(Solution::is_number("2e10".into()));
        assert!(Solution::is_number("-90E3".into()));
        assert!(Solution::is_number("3e+7".into()));
        assert!(Solution::is_number("+6e-1".into()));
        assert!(Solution::is_number("53.5e93".into()));
        assert!(Solution::is_number("-123.456e789".into()));
    }

    #[test]
    pub fn t2() {
        // 部分无效数字列举如下：["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]
        assert!(!Solution::is_number("abc".into()));
        assert!(!Solution::is_number("1a".into()));
        assert!(!Solution::is_number("1e".into()));
        assert!(!Solution::is_number("e3".into()));
        assert!(!Solution::is_number("99e2.5".into()));
        assert!(!Solution::is_number("--6".into()));
        assert!(!Solution::is_number("-+3".into()));
        assert!(!Solution::is_number("95a54e53".into()));
        assert!(!Solution::is_number("e".into()));
        assert!(!Solution::is_number(".".into()));
        assert!(!Solution::is_number("123e".into()));
    }
}