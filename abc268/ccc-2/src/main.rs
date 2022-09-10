use proconio::input;

fn main() {
    input! {
        n:usize,
        p:[usize;n],
    }

    let mut len = vec![0; n];

    p.iter()
        .enumerate()
        .map(|(i, x)| (x + n - i) % n)
        .for_each(|x| len[x] += 1);

    let mut max = 0;

    for i in 0..n {
        let tmp = len[i] + len[(i + 1) % n] + len[(i + 2) % n];
        if tmp > max {
            max = tmp;
        }
    }

    println!("{}", max);
}
