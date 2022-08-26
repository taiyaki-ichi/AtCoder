// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize;n],
        cy: [(usize,usize);m],
    }

    let mut dp=vec![vec![0;n+1];n+1];

    let mut b=vec![0;n+1];
    for &(c,y) in &cy{
        b[c]=y;
    }

    for i in 1..n+1{
        for j in 0..n+1{
            if i<j{
                continue;
            }

            if j==0 {
                dp[i][j]=*dp[i-1].iter().max().unwrap();
            }else{
                dp[i][j]=dp[i-1][j-1]+x[i-1]+b[j];
            }
        }
    }

    // println!("{:?}",dp);
    println!("{}",*dp[n].iter().max().unwrap());
}
