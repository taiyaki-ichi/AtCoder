use proconio::input;

fn main() {
    input! {
        n:usize,
        b:usize,
        k:usize,
        c:[usize;k]
    }

    let m: usize = 10_usize.pow(9) + 7;

    let mut dp = vec![vec![0; b]; n + 1];
    dp[0][0] = 1;

    for n_i in 1..=n {
        for b_i in 0..b {
            for c_i in 0..k {
                let rem = (b_i * 10 + c[c_i]) % b;
                dp[n_i][rem] += dp[n_i - 1][b_i];
                dp[n_i][rem] %= m;
            }
        }
    }

    //println!("{:?}", dp);
    println!("{}", dp[n][0]);
}
