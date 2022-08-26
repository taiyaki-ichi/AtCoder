// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: i64,
        y: i64,
    }

    for i in 0..n + 1 {
        for j in 0..n + 1 {
            let k = n - i - j;

            if k >= 0 && i * 10000 + j * 5000 + k * 1000 == y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
