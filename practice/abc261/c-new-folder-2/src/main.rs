// -*- coding:utf-8-unix -*-

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    }

    let mut num=HashMap::<&String,usize>::new();

    for i in 0..n{
        num.insert(&s[i], 0);
    }
    
    for i in 0..n{
        if num[&s[i]]==0{
            println!("{}",s[i]);
        }else{
            println!("{}({})",s[i],num[&s[i]]);
        }
        num.insert(&s[i], num[&s[i]]+1);
    }
}
