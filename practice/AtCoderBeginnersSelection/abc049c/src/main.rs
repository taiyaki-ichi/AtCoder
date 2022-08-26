// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        s: String
    }

    let mut head=0;

    {
        let is_head_dream = compare_head(&s, head, "dream");
        let is_head_ase = compare_head(&s, head, "erase");
        if !is_head_dream&&!is_head_ase{
            println!("NO");
            return ;
        }

        if is_head_ase{
            head+=2;
        }
    }

    while head<s.len(){
        match is_valid_head(&s, head) {
            Some(next_head) =>{
                head=next_head;
            }
            _ =>{
                println!("NO");
                return ;
            }
        }
    }

    println!("YES");
}

fn compare_head(str1: &String, head: usize, str2: &str) -> bool {
    if str1.len()<head+str2.len() {return false}

    let tmp = &str1[head..head+str2.len()];
    tmp == str2
}

fn calc_er(str1: &String, mut head: usize) -> usize {
    let er = "er";
    loop {
        if str1.len()<head+er.len() {break;}
        let tmp = &str1[head..head+er.len()];
        if tmp != er {
            break;
        }

        head += er.len();
        println!("{}",head);
    }
    head
}

fn is_valid_head(str1: &String, head: usize) -> Option<usize> {
    let is_head_dream = compare_head(str1, head, "dream");
    let is_head_ase = compare_head(str1, head, "ase");

    if !is_head_dream && !is_head_ase {
        return None;
    }

    let er_head = if is_head_dream { head + 5 } else { head + 3 };
    let er_cnt = calc_er(str1, er_head);

    let second_head=er_head+er_cnt*2;

    let is_second_head_dream = compare_head(str1, second_head, "dream");
    let is_second_head_ase = compare_head(str1, second_head, "ase");

    if is_head_dream&&is_second_head_dream&&er_cnt<=1{
        return Some(second_head);
    }else if is_head_dream&&is_second_head_ase&&er_cnt<=2{
        return Some(second_head);
    }else if is_head_ase&&is_second_head_dream&&er_cnt<=1{
        return Some(second_head);
    }else if is_head_ase&&is_second_head_ase&&0<er_cnt&&er_cnt<=2{
        return Some(second_head);
    }else {
        return None;
    }

}
