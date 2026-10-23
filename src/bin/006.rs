// Problem 6: Sum Square Difference
 
// The sum of the squares of the first ten natural numbers is:
// 1^2 + 2^2 + ... + 10^2 = 385
 
// The square of the sum of the first ten natural numbers is:
// (1 + 2 + ... + 10)^2 == 55^2 = 3025
 
// Hence the difference between the sum of the squares of the first ten natural
// numbers and the square of the sum is 3025 - 385 = 2640.
 
// Find the difference between the sum of the squares of the first one hundred
// natural numbers and the square of the sum.
 
fn sum_square_diff(num: i32) -> i32 {
    let mut sum_of_squares = 0;
    let mut square_of_sum = 0;
    for x in 1..=num {
        sum_of_squares += x.pow(2);
        square_of_sum += x;
    }
    square_of_sum = square_of_sum.pow(2);
    square_of_sum - sum_of_squares
}
 
fn main() {
    let start = std::time::Instant::now();
    println!("{}", sum_square_diff(100));
    println!("Elapsed: {:?}", start.elapsed());
}