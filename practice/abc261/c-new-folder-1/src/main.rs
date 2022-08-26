// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [String;n]
    }

    let mut num=Vec::<usize>::new();
    num.resize(n, 0);

    for i in 0..n {
        if num[i]>0{
            continue;
        }

        let mut cnt=0;
        for j in i+1..n{
            if s[i]==s[j] {
                cnt+=1;
                num[j]=cnt;
            }
        }
    }

    for i in 0..n{
        if num[i]==0{
            println!("{}",s[i]);
        }else{
            println!("{}({})",s[i],num[i]);
        }
    }
}
