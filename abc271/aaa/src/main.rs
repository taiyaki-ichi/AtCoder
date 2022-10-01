use proconio::input;

fn main() {
    input! {
        n:usize,
    }

    let a=n/16;
    let b=n%16;

    let f=|n|{
        match n {
            10 => print!("A"),
            11 => print!("B"),
            12 => print!("C"),
            13 => print!("D"),
            14 => print!("E"),
            15 => print!("F"),
            n => print!("{}",n),
        }
    };

    f(a);
    f(b);
    println!();

}
