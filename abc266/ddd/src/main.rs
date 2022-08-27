use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        txa:[(usize,usize,usize);n],
    }

    let mut dp = vec![vec![vec![0; n + 1]; txa[n - 1].0 + 1]; 5 + 1];
    let mut l = HashMap::new();
    for &(t, _, a) in &txa {
        l.insert(t, a);
    }

    for pos_i in 1..=5 {
        for time_i in 1..=txa[n - 1].0 {
            for sunuke_i in 1..=n {
                if time_i >= pos_i
                    && time_i == txa[sunuke_i - 1].0
                    && pos_i - 1 == txa[sunuke_i - 1].1
                {
                    dp[pos_i][time_i][sunuke_i] = std::cmp::max(
                        dp[pos_i][time_i - 1][sunuke_i - 1] + txa[sunuke_i - 1].2,
                        dp[pos_i - 1][time_i - 1][sunuke_i - 1] + txa[sunuke_i - 1].2,
                    );
                }

                dp[pos_i][time_i][sunuke_i] =
                    std::cmp::max(dp[pos_i][time_i][sunuke_i], dp[pos_i][time_i - 1][sunuke_i]);

                dp[pos_i][time_i][sunuke_i] = std::cmp::max(
                    dp[pos_i][time_i][sunuke_i],
                    dp[pos_i - 1][time_i - 1][sunuke_i],
                );

                dp[pos_i][time_i][sunuke_i] = std::cmp::max(
                    dp[pos_i][time_i][sunuke_i],
                    dp[pos_i][time_i - 1][sunuke_i - 1],
                );

                dp[pos_i][time_i][sunuke_i] = std::cmp::max(
                    dp[pos_i][time_i][sunuke_i],
                    dp[pos_i - 1][time_i - 1][sunuke_i - 1],
                );
            }
        }
    }

    //println!("{:?}", dp);
    println!("{}", dp[5][txa[n - 1].0][n]);
}
