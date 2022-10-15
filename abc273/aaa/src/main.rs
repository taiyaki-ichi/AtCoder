use proconio::input;

fn main() {
    input! {
        n:usize,
    }

    println!("{}", f(n));
}

fn f(n: usize) -> usize {
    if n == 0 {
        return 1;
    } else {
        return n * f(n - 1);
    }
}
