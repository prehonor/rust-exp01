#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod cheader;

extern crate libc;

use crate::cheader::hello;

fn test_invoke_cpp() {
    let input = 16;
    let output = unsafe{ hello() };
    println!("rust input :{} hello.cpp output: {}", input, output);
}
fn invoke_cpp() {
    // yew::start_app::<Model>();
    test_invoke_cpp()
}