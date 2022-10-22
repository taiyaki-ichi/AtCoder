use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        c:[Chars;h],
    }

    for w_i in 0..w {
        let mut sum = 0;
        for h_i in 0..h {
            if c[h_i][w_i] == '#' {
                sum += 1;
            }
        }
        print!("{} ", sum);
    }
    println!("");
}
