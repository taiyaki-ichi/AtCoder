// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [u32;n],
    }
    
    d.sort();
    d.dedup();

    println!("{:?}",d.iter().count());
}
