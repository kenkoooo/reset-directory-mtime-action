fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let working_directory = args.get(1).expect("msg");
    println!("{}", working_directory);
}
