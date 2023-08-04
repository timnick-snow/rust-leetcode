#![allow(dead_code)]

struct Solution;

struct Balance {
    left: usize,
    right: usize,
    data: Vec<char>,
}

impl Balance {
    fn new() -> Self {
        Balance {
            left: 0,
            right: 0,
            data: Vec::new(),
        }
    }
    fn push_left(&mut self) {
        self.data.push('(');
        self.left += 1;
    }
    fn push_right(&mut self) {
        self.data.push(')');
        self.right += 1;
    }
    fn pop(&mut self) {
        match self.data.pop() {
            Some('(') => {
                self.left -= 1;
            }
            Some(')') => {
                self.right -= 1;
            }
            _ => (),
        }
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        let mut balance = Balance::new();
        back_trace(&mut balance, n as usize, &mut ans);
        ans
    }
}

fn back_trace(balance: &mut Balance, n: usize, ans: &mut Vec<String>) {
    if balance.data.len() == n * 2 {
        ans.push(balance.data.iter().collect());
    }
    if balance.left < n {
        balance.push_left();
        back_trace(balance, n, ans);
        balance.pop();
    }
    if balance.left > balance.right {
        balance.push_right();
        back_trace(balance, n, ans);
        balance.pop();
    }
}

#[cfg(test)]
mod test {
    use crate::dynamic::generate_parenthesis::Solution;

    #[test]
    fn t1() {
        let vec = Solution::generate_parenthesis(1);
        println!("{:?}", vec);
    }
}