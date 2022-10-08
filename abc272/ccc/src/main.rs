use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }

    let even = a.iter().filter(|&v| v % 2 == 0).sorted().collect_vec();
    let odd = a.iter().filter(|&v| v % 2 != 0).sorted().collect_vec();

    let even_v = if even.iter().count() >= 2 {
        even[even.iter().count() - 1] + even[even.iter().count() - 2]
    } else {
        -1
    };

    let odd_v = if odd.iter().count() >= 2 {
        odd[odd.iter().count() - 1] + odd[odd.iter().count() - 2]
    } else {
        -1
    };

    println!("{}", std::cmp::max(even_v, odd_v));
}
