use proconio::input;

fn main() {
    input! {
        a:f32,
        b:f32,
    }

    let ans = b / a;
    let ans = (ans * 1000_f32).round() / 1000_f32;
    println!("{:.3}", ans);
}
