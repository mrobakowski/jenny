#![feature(proc_macro)]

#[macro_use]
extern crate jenny_derive;

use jenny_derive::jni;

fn main() {
    println!("Hello world");
    let x = foo(1, 2.0);
}

#[jni]
fn foo(x: i64, f: f32) -> f64 {
    x as f64 + f as f64
}