mod rectangle;

use crate::rectangle::{A, B};

fn main() {
    let a = A { a: 42, };
    let b = B { b: 58, };
    println!("Hello, world!  {:?} {:?}", a.a, b.b);
}
