use proconio::input;

fn main() {
    input! {
        n:i64,
    }

    let c = 998244353;

    let mut a = 0;

    if n >= 0 {
        while a * c < n {
            a += 1;
        }
        a -= 1;
    } else {
        while a * c > n {
            a -= 1;
        }
    }

    let hoge=if n - a * c==c{0}else{n - a * c};
    println!("{}", hoge);
}
