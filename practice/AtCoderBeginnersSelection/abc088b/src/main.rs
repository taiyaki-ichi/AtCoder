// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n], 
    }
   
    a.sort();
    a.reverse();

    let mut alice_sum:u32=0;
    let mut bob_sum:u32=0;

    for a_i in 0..n {
        if a_i%2==0{
            alice_sum+=a[a_i];
        }
        else{
            bob_sum+=a[a_i];
        }
    }

    println!("{}",alice_sum-bob_sum);
}
