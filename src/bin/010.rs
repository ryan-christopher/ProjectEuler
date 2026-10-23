//Problem 10: Summation of Primes
 
//The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 
//Find the sum of all the primes below two million.
 
 
fn prime_sum(max_val: i64) -> i64{
    let mut sum: i64 = 0;
    if max_val > 2 {
        sum += 2;
    }
    let mut search_val: i64 = 3;
    while search_val <= max_val{
        if is_prime(search_val){
            sum += search_val;
        }
        search_val += 2;
    }
    sum
}
 
// function to determine if number is prime
fn is_prime(n: i64) -> bool {
    if n % 2 == 0 {
        return false;
    }
    if n == 3 {
        return true;
    }
    let mut search_val = 3;
    let max_val = ((n as f64).sqrt().ceil() as i64) + 1;
    while search_val <= max_val {
        if n % search_val == 0 {
            return false
        }
        search_val += 2;
    }
    return true;
}
 
fn main() {
    let start = std::time::Instant::now();
    println!("{:?}", prime_sum(2000000));
    println!("Elapsed: {:?}", start.elapsed());
}