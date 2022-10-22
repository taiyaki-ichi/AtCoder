use itertools::Itertools;
use proconio::input;
 
fn main() {
    input! {
        n:usize,
        x:i32,
        y:i32,
        a:[i32;n],
    }
    let a = a.into_iter().collect_vec();
    let ans = check(0, 0, &a, x) && check(0, 1, &a, y);
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
 
fn check(curr: i32, i: usize, a: &Vec<i32>, pos: i32) -> bool {
    //println!("{} {} {}", curr, i, pos);
    if i >= a.iter().count() {
        return curr == pos;
    }
    if i == 0 {
        return check(curr + a[i], i + 2, a, pos);
    } else {
        return check(curr + a[i], i + 2, a, pos) || check(curr - a[i], i + 2, a, pos);
    }
}