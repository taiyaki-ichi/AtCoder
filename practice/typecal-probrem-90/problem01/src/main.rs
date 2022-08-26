use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        l:usize,
        k:usize,
        a:[usize;n],
    }

    let mut left = 1;
    let mut right = l;

    let mut delta_data = a.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();
    delta_data.insert(0, a[0]);
    delta_data.push(l - a[n - 1]);

    while right - left > 1 {
        let center = (left + right) / 2;

        let (num, _) = delta_data.iter().fold((0, 0), |(num, cnt), x| {
            if cnt + x >= center {
                (num + 1, 0)
            } else {
                (num, cnt + x)
            }
        });

        if num > k {
            left = center;
        } else {
            right = center;
        }
    }

    println!("{}", left);
}
