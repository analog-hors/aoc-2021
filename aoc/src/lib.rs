#[macro_export]
macro_rules! main {
    () => {
        fn main() {
            let stdin = std::io::stdin();
            let input = stdin.lock();
            match std::env::args().skip(1).next().as_deref() {
                Some("1") => println!("{}", part_1(input)),
                Some("2") => println!("{}", part_2(input)),
                _ => eprintln!("Expected AoC part as argument (1 or 2)")
            }
        }
    }
}
