mod some {
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_add_one() {
            assert_eq!(3, add_one(2));
        }
    }
    fn add_one(x: isize) -> isize {
        x + 1
    }
}
fn main() {
    println!("Hello, world!");
}
