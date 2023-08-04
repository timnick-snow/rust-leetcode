/*
周期: 2n-2

0   4   8     12
1 3 5 7 9  11 13
2   6   10
分析: p=4, len=14, 14/4=3..2
第0行: 3+1=4, 4*i
第1行: 3*2+1=6, 2*i+1
第2行: 3, 2+4*i

0     6        12
1   5 7     11 13
2 4   8  10    14  16
3     9        15
分析: p=6, len=16, 16/6=2..4
第0行: 2+1=3, 6*i
第1行: i是偶数: 1+6*i/2, i是奇数: 5+6*i/2
第2行: i是偶数: 2+6*i/2, i是奇数: 4+6*i/2
last(3): 2+1=3, 3+6*i

 */

pub fn convert(s: String, n: i32) -> String {
    if n == 1 { return s; }
    // p周期长度  full完整周期数 rest是剩余数
    let p = (2 * n - 2) as usize;
    let full = s.len() as usize / p;
    let rest = s.len() as usize % p;

    // 第0行
    let size = full + (if rest >= 1 { 1 } else { 0 });
    let mut ans: String = (0..size)
        .map(|i| s.chars().nth(p * i).unwrap())
        .collect();
    // 1..n-1
    (1..(n - 1) as usize)
        .map(|r| {
            let size = full * 2 + {
                if rest < r + 1 {
                    0
                } else if rest >= 2 * n as usize - r - 1 {
                    2
                } else {
                    1
                }
            };
            let n_row = (0..size)
                .map(|i| {
                    if i & 1 == 0 {
                        // r=1 p=4
                        // i=0 y=1, i=2 y=5
                        s.chars().nth(r + p * (i / 2)).unwrap()
                    } else {
                        // i=1 y=3+
                        s.chars().nth((p - r) + p * (i / 2)).unwrap()
                    }
                })
                .collect::<String>();
            println!("the {} row is {}", r, n_row);
            n_row
        })
        .for_each(|row| ans.push_str(&row));
    // n-1 行
    let size = full + (if rest >= n as usize { 1 } else { 0 });
    let last: String = (0..size)
        .map(|i| s.chars().nth((n - 1) as usize + p * i).unwrap())
        .collect();
    ans.push_str(&last);
    ans
}

#[cfg(test)]
mod test {
    use crate::string::n_row_convert::convert;

    #[test]
    fn example1() {
        let string = convert("PAYPALISHIRING".to_string(), 3);
        assert_eq!(&string, "PAHNAPLSIIGYIR");
    }

    #[test]
    fn example2() {
        let string = convert("A".to_string(), 2);
        assert_eq!(&string, "A");
    }

    #[test]
    fn example3() {
        let string = convert("ABCDE".to_string(), 4);
        assert_eq!(&string, "ABCED");
    }
}