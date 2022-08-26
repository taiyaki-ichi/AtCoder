// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a:u32,
        b:u32,
        c:u32,
        x:u32,
    }
    
    let mut cnt=0;
    
    for a_i in 0..a+1 {
        for b_i in 0..b+1 {
            for c_i in 0..c+1 {
                if 500*a_i+100*b_i+50*c_i==x {
                    cnt+=1;
                }
            }
        }
    }

    println!("{}",cnt);
}
