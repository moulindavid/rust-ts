fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("The file name to be passed in");

    let file = std::fs::read_to_string(file_name).expect("unable to the file to string");

    file.lines().for_each(|line| println!("{}", line))
}
