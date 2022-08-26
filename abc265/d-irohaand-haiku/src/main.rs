use proconio::input;

fn main() {
    input! {
        n:usize,
        p:usize,
        q:usize,
        r:usize,
        a:[usize;n],
    }

    let mut dp = vec![vec![false; n]; 3];
    let pqr = [p, q, r];

    for i in 0..3 {
        for j in 0..n {
            let mut sum = 0;
            let mut k = j;
            while sum < pqr[i] {
                sum += a[k];
                if k == 0 {
                    break;
                } else {
                    k -= 1;
                }
            }
            //println!("{}",sum);

            if sum == pqr[i] {
                if i == 0 {
                    dp[i][j] = true;
                } else if k > 0 && dp[i - 1][k] {
                    dp[i][j] = true;
                }
            }
        }
    }

    //println!("{:?}",dp);

    if !dp[2].iter().all(|x| !*x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
