mod parser;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    static EX1: &str = include_str!("example.txt");

    #[test]
    fn example1() {
        let (_, readings) = parse(EX1).unwrap();
    }
}
