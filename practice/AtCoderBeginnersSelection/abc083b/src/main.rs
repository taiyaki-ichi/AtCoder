// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }

    let mut sum = 0;

    for n_i in 1..n + 1 {
        let tmp = calc_all_digit_sum(n_i);
        if a <= tmp && tmp <= b {
            sum += n_i;
        }
    }

    println!("{}", sum);
}

fn calc_all_digit_sum(mut num: u32) -> u32 {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}
