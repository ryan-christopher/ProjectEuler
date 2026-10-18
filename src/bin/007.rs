// Problem 7: 10,001st Prime
 
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that
// the 6th prime is 13.
 
// What is the 10,001st prime number?
 
fn is_prime(n: i64) -> bool {
    if n % 2 == 0 {
        return false;
    }
    if n == 3 {
        return true;
    }
    let mut search_val = 3 as i64;
    let max_val = ((n as f64).sqrt().ceil() as i64) + 1;
    while search_val <= max_val {
        if n % search_val == 0 {
            return false;
        }
        search_val += 2;
    }
    return true;
}
 
fn find_nth_prime(n: i64) -> i64{
    let mut curr_prime = 1;
    let mut x = 1;
    while curr_prime < n {
        x += 2;
        if is_prime(x) {
            curr_prime += 1;
        }
    }
    x
}
 
fn main() {
    println!("{}", find_nth_prime(10001));
}