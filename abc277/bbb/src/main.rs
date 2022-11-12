use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }

    let ans1 = s
        .iter()
        .all(|s| s[0] == 'H' || s[0] == 'D' || s[0] == 'C' || s[0] == 'S');

    let ans2 = s.iter().all(|s| {
        s[1] == 'A'
            || s[1] == '2'
            || s[1] == '3'
            || s[1] == '4'
            || s[1] == '5'
            || s[1] == '6'
            || s[1] == '7'
            || s[1] == '8'
            || s[1] == '9'
            || s[1] == 'T'
            || s[1] == 'J'
            || s[1] == 'Q'
            || s[1] == 'K'
    });

    let ans3 = s.iter().sorted().dedup().count() == s.iter().count();

    println!("{}", if ans1 && ans2 && ans3 { "Yes" } else { "No" })
}
