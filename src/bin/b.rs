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
        n: i64,
        a: i64,
        b: i64
    };

    let mut ans = 0i64;

    let x = n / (a+b);
    ans += x * a;

    let m = n - x * (a+b);

    if m <= a {
        ans += m
    }
    else {
        ans += a
    }

    println!("{}", ans);

}
