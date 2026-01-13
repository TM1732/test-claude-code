use std::io::{self, Write};

/// Main entry point of the Fibonacci sequence program
fn main() {
    println!("=== Fibonacci Sequence Generator ===\n");

    // Prompt the user to enter how many Fibonacci numbers to display
    print!("Enter the number of Fibonacci numbers to display: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

    // Read user input from standard input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Parse the input string into an unsigned integer
    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Please enter a valid positive number");
            return;
        }
    };

    // Display the Fibonacci sequence
    println!("\nThe first {} Fibonacci numbers are:", n);
    print_fibonacci_sequence(n);
}

/// Prints the first n numbers of the Fibonacci sequence
///
/// # Arguments
/// * `n` - The count of Fibonacci numbers to generate and display
fn print_fibonacci_sequence(n: u32) {
    // Handle edge case: if user requests 0 numbers
    if n == 0 {
        println!("No numbers to display.");
        return;
    }

    // Initialize the first two Fibonacci numbers
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    // Generate and print each Fibonacci number
    for i in 0..n {
        // Print the current Fibonacci number with its position
        println!("F({}) = {}", i, a);

        // Calculate the next Fibonacci number
        // The next number is the sum of the current two numbers
        let next = a + b;

        // Shift the sequence: a becomes b, and b becomes the next value
        a = b;
        b = next;
    }
}

/// Alternative implementation: Returns a vector containing the Fibonacci sequence
/// This function is provided as an alternative approach
///
/// # Arguments
/// * `n` - The count of Fibonacci numbers to generate
///
/// # Returns
/// A vector containing the first n Fibonacci numbers
#[allow(dead_code)]
fn generate_fibonacci_vector(n: u32) -> Vec<u64> {
    // Create an empty vector to store the sequence
    let mut fibonacci_sequence = Vec::new();

    // Handle edge case
    if n == 0 {
        return fibonacci_sequence;
    }

    // Initialize the first two Fibonacci numbers
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    // Generate each Fibonacci number and add it to the vector
    for _ in 0..n {
        fibonacci_sequence.push(a);

        // Calculate the next number and update variables
        let next = a + b;
        a = b;
        b = next;
    }

    fibonacci_sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_generation() {
        // Test that the function generates the correct Fibonacci sequence
        let result = generate_fibonacci_vector(10);
        let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_sequence() {
        // Test edge case: requesting 0 numbers
        let result = generate_fibonacci_vector(0);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_single_number() {
        // Test requesting just one number
        let result = generate_fibonacci_vector(1);
        assert_eq!(result, vec![0]);
    }
}
