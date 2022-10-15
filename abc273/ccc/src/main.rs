use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }

    let a = a.into_iter().sorted().collect_vec();

    let mut cnt = vec![0; n];
    for i in (0..n - 1).rev() {
        if a[i] == a[i + 1] {
            cnt[i] = cnt[i + 1];
        } else {
            cnt[i] = cnt[i + 1] + 1;
        }
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        ans[cnt[i]] += 1;
    }

    for i in 0..n {
        println!("{}", ans[i]);
    }
}
