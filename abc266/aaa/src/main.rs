use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }

    print!("{}", s.iter().nth(s.len() / 2).unwrap());
}
