fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [12, 5, 9, 15, 22, 30, 7, 18, 25, 3];

    // For loop: Analyze each number
    for &num in &numbers {
        if num % 15 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

    // While loop: Sum all numbers
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Loop: Find largest number
    let mut largest = numbers[0];
    for &num in &numbers {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}