use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }

    let MOD = 10_usize.pow(9) + 7;
    let atcoder = "atcoder".to_string();

    let mut dp = vec![vec![0; atcoder.len() + 1]; n + 1];

    for s_i in 0..=n {
        dp[s_i][0] = 1;
    }

    for s_i in 1..=n {
        for atcoder_i in 1..=atcoder.len() {
            if s[s_i - 1] == atcoder.chars().nth(atcoder_i - 1).unwrap() {
                dp[s_i][atcoder_i] += dp[s_i - 1][atcoder_i - 1];
            }

            dp[s_i][atcoder_i] += dp[s_i - 1][atcoder_i];
            dp[s_i][atcoder_i] %= MOD;
        }
    }

    //println!("{:?}", dp);
    println!("{}", dp[n][atcoder.len()]);
}
