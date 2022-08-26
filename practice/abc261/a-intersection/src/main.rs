// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        l1: u32,
        r1: u32,
        l2: u32,
        r2:u32,
    }

    if r1 <= l2 || r2 <= l1 {
        println!("0");
        return;
    }

    let l = std::cmp::max(l1, l2);
    let r = std::cmp::min(r1, r2);

    println!("{}", r - l);
}
