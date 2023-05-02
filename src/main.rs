#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(item: &mut Item) {
    item.count += 1;
}

fn print_all(items: &mut Vec<Item>) {
    for item in items {
        println!("{:?}", item);
    }
}

fn main() {
    let mut items = vec![Item { count: 1 }];

    let first = items.first_mut();
    println!("{:?}", first);

    print_all(&mut items);
    // If we do this we break borrow rules, there can only be one mutable reference and no
    // immutuble references
    //   println!("{:?}", first);
}
