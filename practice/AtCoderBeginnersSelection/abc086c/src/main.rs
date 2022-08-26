// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        mut plan: [(i32, i32, i32); n],  // Vec<(i32, i32, i32)>
    }

    for plan_i in 0..n {
        let prev_plan = if plan_i == 0 {
            (0, 0, 0)
        } else {
            plan[plan_i - 1]
        };
        let d = (prev_plan.1 - plan[plan_i].1).abs() + (prev_plan.2 - plan[plan_i].2).abs();
        let t = (prev_plan.0 - plan[plan_i].0).abs();

        if d > t || (t - d) % 2 != 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
