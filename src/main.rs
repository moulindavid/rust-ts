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

    let file_odd = std::fs::read_to_string("lines_odd").unwrap();
    file_odd
        .lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|(_, line)| println!("{}", line));

    file
        .lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|line| println!("{}", line.1));

}
