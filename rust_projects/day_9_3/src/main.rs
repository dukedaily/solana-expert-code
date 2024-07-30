#[derive(Debug)]
struct Foo {
    bar: i32,
}

pub fn main() {
    let foo = Foo { bar: 3 };
    println!("{:?}", foo);
}
