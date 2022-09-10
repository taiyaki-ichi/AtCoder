use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        s:[Chars;n],
        t:[Chars;m],
    }

    for t_str in t{
        let a=t_str.split(|c|*c=='_').into_iter().map(|c|
            for i in 0..n{
                if c==s[i]{
                    i
                }
            }
            -1
        ).collect();
    }

}
