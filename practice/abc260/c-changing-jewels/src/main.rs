use std::{convert::TryInto, usize, vec};

use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        y:usize,
    }

    let mut r = [0; 11];
    let mut b = [0; 11];

    b[1] = 1;

    for i in 2..n + 1 {
        b[i] = r[i - 1] + b[i - 1] * y;
        r[i] = r[i - 1] + b[i] * x;
    }

    println!("{}",r[n]);
}
