// day_9_1/src/main.rs
use day_9_1::foo_bar_attribute;

#[foo_bar_attribute]
struct MyStruct {
    baz: i32,
}

fn main() {
    let demo = MyStruct::default();
    println!("struct is {:?}", demo);

    let double_foo = demo.double_foo();
    println!("double foo is {}", double_foo);
}