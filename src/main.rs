fn main() {
    let foo: Vec<_> = vec![1, 2 ,3]
        .iter()
        .map(|x| x + 1)
        .collect();

    println!("{:?}", foo);

    let data = vec![1, 2, 3];
    let mut bar = data
        .iter()
        .map(|x| x + 1);

    let mut new_vector = vec![];

    while let Some(x) = bar.next() {
        new_vector.push(x)
    }

    println!("{:?}", foo);

    let file = std::fs::read_to_string("lines").unwrap();
   
    file
        .lines()
        .for_each(|line| println!("{}", line));
}
