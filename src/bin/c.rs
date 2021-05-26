use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        a: i32,
        b: i32
    };

    let mut ans= -1;

    for x in 0..=1000000{
        if x * 8 / 100 == a && x* 10/100 == b{
            ans = x;
            break;
        }
    }

    println!("{}", ans);
}
