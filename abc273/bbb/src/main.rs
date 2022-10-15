use proconio::input;

fn main() {
    input! {
        mut x:u64,
        k:u32,
    }

    for i in 0..k {
        let size = x.to_string().chars().count();
        if i as usize>=size{
            println!("0");
            return;
        }
        let v = x.to_string().chars().nth(size - 1 - i as usize);
        // println!("{:?}", v);
        if v.is_none() {
            println!("0");
            return;
        } else {
            let vv = v.unwrap().to_digit(10).unwrap() as u64;
            x -= vv * 10_i64.pow(i) as u64;

            if vv >= 5 {
                x += 10_i64.pow(i + 1) as u64;
            }
        }
        // println!("{}", x);
    }

    println!("{}", x);
}
