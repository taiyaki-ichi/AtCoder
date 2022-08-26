// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a:[Chars;n],
    }

    for j in 0..n {
        for i in 0..n {
            let is_correct = if i == j {
                a[j][i] == '-'
            } else if a[j][i] == 'W' {
                a[i][j] == 'L'
            } else if a[j][i] == 'L' {
                a[i][j] == 'W'
            } else if a[j][i] == 'D' {
                a[i][j] == 'D'
            } else {
                false
            };

            if !is_correct {
                println!("incorrect");
                return;
            }
        }
    }

    println!("correct");
}
