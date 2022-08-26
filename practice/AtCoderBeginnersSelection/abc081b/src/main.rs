// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
        mut a: [u32; n],
    }

    let mut cnt=0;
    while a.iter().all(|x|x%2==0) {
        a=a.iter().map(|x|x/2).collect();
        cnt+=1;
    }
    println!("{}", cnt);
}
