use proconio::input;

fn main() {
    input! {
        n:usize,
        p:[usize;n],
    }

    let mut ans = 0;
    for ii in 0..n {
        let tmp = p
            .iter()
            .enumerate()
            .filter(|&(i, x)| {
                (*x + ii) % n == (i + n - 1) % n
                    || (*x + ii) % n == i
                    || (*x + ii) % n == (i + 1) % n
            })
            .count();

        if tmp > ans {
            ans = tmp;
        }
    }

    println!("{}", ans);
}
