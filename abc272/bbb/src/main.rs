use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
    }

    let mut cnt = vec![vec![0; n]; n];

    for _ in 0..m {
        input! {
            k:usize,
            m:[usize;k],
        }

        for m1 in &m {
            for m2 in &m {
                cnt[m1 - 1][m2 - 1] += 1;
                cnt[m2 - 1][m1 - 1] += 1;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if i != j && cnt[i][j] == 0 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
