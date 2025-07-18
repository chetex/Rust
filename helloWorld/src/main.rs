fn main() {
    println!("Hello, world!");

    println!("The sum of 2 and 3 is: {}", add(2, 3));
    println!("The sum of 5 and 7 is: {}", add(5, 7));
    println!("The sum of -1 and 1 is: {}", add(-1, 1));
}

/**
 * This is a simple Rust program that defines a function to add two integers.
 * The main function demonstrates the usage of the add function by printing the results.
 * The add function takes two i32 parameters and returns their sum.
 */
fn add(a: i32, b: i32) -> i32 {
    a + b
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 3), 5);
    }
}