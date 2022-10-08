use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[i64;n],
    }

    let mut a = a.into_iter().collect_vec();

    for _ in 0..m {
        a = a
            .into_iter()
            .enumerate()
            .map(|(i, a_v)| i as i64 + a_v + 1)
            .collect_vec();

        

        println!("{}", ans);
    }
}
