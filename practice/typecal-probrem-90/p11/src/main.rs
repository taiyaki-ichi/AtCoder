use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        dcs:[(usize,usize,usize);n],
    }

    let dcs = std::iter::once(&(0, 0, 0))
        .chain(dcs.iter().sorted())
        .collect_vec();

    let &&(lim, _, _) = dcs.iter().last().unwrap();
    let mut dp = vec![vec![0; n + 1]; 5001];

    for day_i in 1..=lim {
        for job_i in 1..=n {
            dp[day_i][job_i] = std::cmp::max(dp[day_i][job_i - 1], dp[day_i - 1][job_i]);
            if dcs[job_i].1 <= day_i && day_i <= dcs[job_i].0 {
                dp[day_i][job_i] = std::cmp::max(
                    dp[day_i][job_i],
                    dp[day_i - dcs[job_i].1][job_i - 1] + dcs[job_i].2,
                );
            }
        }
    }

    println!("{}", dp[lim][n]);
}
