use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::collections::VecDeque;

fn main() {
    // let source = AutoSource::from("ab
    // 4
    // 2 1 p
    // 1
    // 2 2 c
    // 1");
    input!{
        // from source,
        s: Chars,
        q: usize
    };

    let mut deq = VecDeque::from(s);

    let mut flg = true;

    for _ in 0..q{
        input! {
            t: i32
        }

        if t == 1{
            flg = !flg;
        }
        else {
            input! {
                f: i32,
                c: char
            }
            if f == 1{
                if flg{
                    deq.push_front(c)
                }
                else {
                    deq.push_back(c)
                }
            }
            else {
                if !flg{
                    deq.push_front(c)
                }
                else {
                    deq.push_back(c)
                }
            }
        }
    }

    if flg{
        println!("{}", deq.iter().collect::<String>());
    }
    else {
        println!("{}", deq.iter().rev().collect::<String>());
    }
}
