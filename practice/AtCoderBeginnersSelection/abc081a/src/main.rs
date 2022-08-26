// -*- coding:utf-8-unix -*-
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }

    let sum=s.iter().filter(|x|**x=='1').count();

    println!("{:?}", sum);
}
