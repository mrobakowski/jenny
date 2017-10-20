#![feature(proc_macro)]

extern crate jenny_derive;
extern crate jenny;

use jenny_derive::jni;

#[jni(class = "com.mrobakowski.jenny.HelloWorld")]
fn foo(x: i64, f: f32) -> f64 {
    x as f64 + f as f64
}

#[jni(class = "com.mrobakowski.jenny.HelloWorld", name = "helloWorld")]
fn hello_world() {
    println!("Hello from Rust!")
}

// This is here just as a test to check if the code compiles
#[allow(dead_code)]
fn test() {
    let _ = mod_Java_com_mrobakowski_jenny_HelloWorld_foo::Java_com_mrobakowski_jenny_HelloWorld_foo;
    let _ = mod_Java_com_mrobakowski_jenny_HelloWorld_helloWorld::Java_com_mrobakowski_jenny_HelloWorld_helloWorld;
}