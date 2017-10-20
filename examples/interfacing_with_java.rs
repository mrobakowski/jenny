#![feature(proc_macro)]

extern crate jenny_derive;
extern crate jenny;

use jenny_derive::jni;

fn main() {
    println!("Hello world");

    let (a, b, c, d) = unsafe { std::mem::uninitialized() };
    use mod_Java_rust_jenny_Foo_foo::Java_rust_jenny_Foo_foo;

    let _ = Java_rust_jenny_Foo_foo(a, b, c, d);
}

#[jni]
fn foo(x: i64, f: f32) -> f64 {
    x as f64 + f as f64
}