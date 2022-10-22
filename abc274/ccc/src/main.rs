use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }

    let mut dp = vec![(-1, 0); (2 * n + 1) as usize];

    dp[0] = (1, 0);
    for i in 0..n {
        let (p_i, p_num) = dp[a[i] - 1];
        let c1 = 2 * (i + 1);
        let c2 = 2 * (i + 1) + 1;
        if c1 <= 2 * n + 1 {
            dp[c1 - 1] = (p_i, p_num + 1);
        }
        if c2 <= 2 * n + 1 {
            dp[c2 - 1] = (p_i, p_num + 1);
        }
    }

    for (_, ans) in dp {
        println!("{}", ans);
    }
}
