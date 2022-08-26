// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: usize,
        bc: (usize,usize),
        s: String,
    }
    
    let sum=a+bc.0+bc.1;

    println!("{} {}",sum,s);
}
