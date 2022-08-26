use ndarray::Array2;
use proconio::input;

fn main() {
    input! {
        n:usize,
        b:usize,
        k:usize,
        c:[usize;k],
    }

    let MOD = 10_usize.pow(9) + 7;

    let mut m: Array2<usize> = Array2::zeros((k, k));

    for b_i in 0..b {
        for c_i in 0..k {
            let rem = (b_i * 10 + c[c_i]) % b;
            m[(rem, b_i)] = 1;
        }
    }


}
