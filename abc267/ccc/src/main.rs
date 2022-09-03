use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[i64;n],
    }

    let mut result: i64 = a
        .iter()
        .take(m)
        .enumerate()
        .fold(0, |acc, (i, v)| acc + (i as i64 + 1) * (*v));
    let mut prev = result;

    let mut sum = vec![0; n + 1];
    for i in 1..n {
        sum[i] = sum[i - 1] + a[i - 1];
    }

    for i in 1..n - m + 1 {
        /*
        for j in 0..m {
            prev -= a[i - 1 + j];
        }
        */
        prev -= sum[i + m - 1] - sum[i - 1];
        prev += m as i64 * a[i + m - 1];

        // println!("{}", prev);

        if prev > result {
            result = prev;
        }
    }

    println!("{}", result);
}
