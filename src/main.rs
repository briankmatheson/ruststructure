#[derive(Debug)]
struct A {
    a: i32,
}

#[derive(Debug)]
struct B {
    b: i32,
}

fn main() {
    let a = A { a: 42, };
    let b = B { b: 58, };
    println!("Hello, world!  {:?} {:?}", a.a, b.b);
}
