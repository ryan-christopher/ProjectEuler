// Problem 4: Largest Palindrome Project

// A palindromic number reads the same both ways. The largest palindrome made from the 
// product of two 2-digit numbers is 9009 = 91 x 99.

// Find the largest palindrome made from the product of two 3-digit numbers.

// function to determine if a number is a palindrome
fn is_palindrome(num: i64) -> bool {
    // convert to string
    let num_string = num.to_string();
    // store in vector
    let num_vec = num_string.split("").collect::<Vec<_>>();
    // start on either ends of the vector, check if values are the same
    let (mut start, mut end) = (1, num_vec.len() - 2);
    while start < end {
        // if any do not match, return false
        if num_vec[start] != num_vec[end] {
            return false
        }
        start += 1;
        end -= 1;
    }
    return true
}

// finds the largest palindromic number made from the products 
// of two numbers that are each digit_num in length
fn largest_palindrome(digit_num: u32) -> i64{
    let (mut digit1, mut digit2) = (0, 0);
    let mut largest_pal: i64 = 0;
    // max val that is digit_num number of digits
    let max_val = 1 * (10i32.pow(digit_num)) - 1;
    // iterate through the combinations of numbers that are digit_num in length
    for x1 in 0..=max_val / 10 {
        for x2 in 0..=max_val / 10 {
            let product: i64 = (max_val - x1) as i64 * (max_val - x2) as i64;
            if is_palindrome(product) {
                if max_val - x1 >= digit1 || max_val - x2 >= digit2 {
                    if product > largest_pal {
                        largest_pal = product;
                        (digit1, digit2) = (max_val - x1, max_val - x2);
                    }
                }
                // end if both values that are multiplied are less than the
                // current max palindromic number factors
                else if max_val - x1 < digit1 && max_val - x2 < digit2{
                    break
                }
            }
        }
    }
    return largest_pal;

}

fn main() {
    println!("{:?}", largest_palindrome(3));
}