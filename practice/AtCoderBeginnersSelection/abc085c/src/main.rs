// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        y: usize,
    }

    let impossible_value = (-1, -1, -1);

    let mut dp = Vec::<(i32, i32, i32)>::new();
    dp.resize((n + 1) * (y / 1000 + 1), impossible_value);

    let get_db_index = |n_i: usize, y_i: usize| -> usize { n_i + y_i * (n + 1) };

    if y / 1000 >= 10 {
        dp[get_db_index(1, 10)] = (1, 0, 0);
    }
    if y / 1000 >= 5 {
        dp[get_db_index(1, 5)] = (0, 1, 0);
    }
    if y / 1000 >= 1 {
        dp[get_db_index(1, 1)] = (0, 0, 1);
    }
    for y_i in 2..y / 1000 + 1 {
        for n_i in 2..n + 1 {
            if y_i >= 10 && dp[get_db_index(n_i - 1, y_i - 10)] != impossible_value {
                dp[get_db_index(n_i, y_i)] = dp[get_db_index(n_i - 1, y_i - 10)];
                dp[get_db_index(n_i, y_i)].0 += 1;
            } else if y_i >= 5 && dp[get_db_index(n_i - 1, y_i - 5)] != impossible_value {
                dp[get_db_index(n_i, y_i)] = dp[get_db_index(n_i - 1, y_i - 5)];
                dp[get_db_index(n_i, y_i)].1 += 1;
            } else if y_i >= 1 && dp[get_db_index(n_i - 1, y_i - 1)] != impossible_value {
                dp[get_db_index(n_i, y_i)] = dp[get_db_index(n_i - 1, y_i - 1)];
                dp[get_db_index(n_i, y_i)].2 += 1;
            }
        }
    }

    //println!("{}", get_db_index(n, y / 1000));
    println!("{} {} {}", dp[get_db_index(n, y / 1000)].0,dp[get_db_index(n, y / 1000)].1,dp[get_db_index(n, y / 1000)].2);
}
