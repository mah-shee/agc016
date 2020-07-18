#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::{max, min};
#[fastout]
fn main() {
    input! {
        s: Chars
    }
    let mut ans = s.len() - 1;
    for i in b'a'..=b'z' {
        let u = check(s.clone(), i as char);
        ans = min(u, ans);
    }
    println!("{}", ans);
}
fn check(s: Vec<char>, c: char) -> usize {
    let mut a = 0;
    let mut pre = 0;
    for i in 0..s.len() {
        if s[i] == c {
            a = max(a, pre);
            pre = 0;
        } else {
            pre += 1;
        }
    }
    max(a, pre)
}
