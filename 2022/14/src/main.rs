mod parser;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    static EX: &str = include_str!("example.txt");
}
