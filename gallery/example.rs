// This is a comment!

#![allow(dead_code, unused)]

use std::collections::HashMap;

struct Structure {
    value: i32,
}

enum Enumeration {
    Foo,
    Bar,
}

fn main() {
    let string: &str = "Hello, World!";
    let character: char = 'I';
    let integer: i64 = 42;
    let number: f32 = 6.28;
    let boolean: bool = true;
    let structure: Structure = Structure { value: 10 };

    let _ = size_of_val(&structure);
    println!("{character} say: {}", string);
}
