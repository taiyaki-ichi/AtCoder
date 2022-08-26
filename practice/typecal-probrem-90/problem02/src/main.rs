use proconio::input;

fn main() {
    input! {
        n:usize,
    }

    for bit in (0..(1 << n)).rev() {
        let item = (0..n)
            .rev()
            .map(|i| if (bit & (1 << i)) != 0 { 1 } else { -1 });

        let mut sum = 0;
        for i in item {
            sum += i;

            if sum < 0 {
                break;
            }
        }

        if sum != 0 {
            continue;
        }

        (0..n)
            .rev()
            .map(|i| if (bit & (1 << i)) != 0 { true } else { false })
            .for_each(|i| if i { print!("(") } else { print!(")") });

        print!("\n");
    }
}
