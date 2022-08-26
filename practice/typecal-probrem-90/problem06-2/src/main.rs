use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        k:usize,
        s:Chars,
    }

    let hoge = s
        .iter()
        .enumerate()
        .sorted_by_key(|&(i, c)| (c, i))
        .map(|(_, c)| c)
        .collect::<String>();
    println!("{}", hoge);

    let (_, str) = s.iter().enumerate().sorted_by_key(|&(i, c)| (c, i)).fold(
        (0, String::new()),
        |(prev_i, s), (i, c)| {
            println!("{} {}", prev_i, s);
            if s.len() < k && i > prev_i && n - i > k - s.len() {
                (i, s + &c.to_string())
            } else {
                (prev_i, s)
            }
        },
    );

    println!("{}", str);
}
