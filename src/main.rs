#[derive(Debug)]
struct MyStruct {
    age: usize,
}

fn main() {
    let item = MyStruct { age: 0 };

    let items: Vec<&MyStruct> = vec![&item];
}
