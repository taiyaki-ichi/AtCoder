use proconio::input;

fn main() {
    input! {
        x:usize,
        y:usize,
        mut n:usize,
    }

    let mut sum = 0;
    if x * 3 > y {
        while n >= 3 {
            sum += y;
            n -= 3;
        }
        sum += n * x;
    } else {
        sum = n * x;
    }

    println!("{}", sum);
}
