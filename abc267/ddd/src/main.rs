use proconio::input;
 
fn main() {
    input! {
        n:usize,
        m:usize,
        a:[i128;n],
    }
 
    let mut dp = vec![vec![std::i128::MIN+100000; m + 1]; n + 1];
 
    for n_i in 1..m {
        dp[1][n_i] = a[0];
    }
 
    for p_i in 2..=n {
        for n_i in 1..=m {
            dp[p_i][n_i] = std::cmp::max(
                dp[p_i - 1][n_i - 1],
                dp[p_i - 1][n_i - 1] + n_i as i128 * a[p_i as usize - 1],
            );
            dp[p_i][n_i] = std::cmp::max(dp[p_i][n_i], dp[p_i - 1][n_i]);
            dp[p_i][n_i] = std::cmp::max(dp[p_i][n_i], dp[p_i][n_i - 1]);
        }
    }
 
    // println!("{:?}",dp);
    println!("{}", dp[n][m]);
}