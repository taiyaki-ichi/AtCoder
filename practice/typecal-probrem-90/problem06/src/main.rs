use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        k:usize,
        s:Chars,
    }

    let s = (0_usize..1 << n)
        .map(|i| {
            (0..n)
                .filter(|x| (i & (1_usize << x)) != 0)
                .map(|x| s[x])
                .into_iter()
                .collect::<String>()
        })
        .filter(|s| s.len() == k)
        .sorted()
        .take(1)
        .collect::<String>();

    println!("{}",s);
}
