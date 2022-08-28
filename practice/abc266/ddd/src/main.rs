use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        txa:[(usize,usize,usize);n],
    }

    let to_len: HashMap<(usize, usize), usize> = txa.iter().map(|&(t, x, a)| ((t, x), a)).collect();

    let mut dp = vec![vec![0; txa[txa.len() - 1].0 + 1]; 5];

    for time_i in 1..=txa[txa.len() - 1].0 {
        for pos_i in 0..5 {
            if pos_i > time_i {
                continue;
            }

            dp[pos_i][time_i] = dp[pos_i][time_i - 1];

            if pos_i > 0 {
                dp[pos_i][time_i] = std::cmp::max(dp[pos_i][time_i], dp[pos_i - 1][time_i - 1]);
            }
            if pos_i < 4 {
                dp[pos_i][time_i] = std::cmp::max(dp[pos_i][time_i], dp[pos_i + 1][time_i - 1]);
            }

            if let Some(l) = to_len.get(&(time_i, pos_i)) {
                dp[pos_i][time_i] += l;
            }
        }
    }

    println!("{}", dp.iter().map(|v| v.last()).flatten().max().unwrap());
}
