// Problem 21: Amicable Numbers

// Let d(n) be defined by the sum of proper divisors of n (numbers less than n which divide evenly into n).
// If d(a) = b and d(b) = a, where a != b, then a and b are an amicable pair and each of a and b are called amicable numbers.

// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55, and 110; therefore d(220) = 284.
// The proper divisors of 284 are 1, 2, 4, 71, and 142; so d(284) = 220.

// Evaluate the sum of all amicable numbers under 10,000.

fn sum_amicable_nums(val: i32) -> i32 {
    let mut amicable_sum = 0;
    let mut known_amicables: Vec<i32> = Vec::new();
    for a in 1..=val {
        let da = find_divisor_sum(a);
        if !known_amicables.contains(&a) && !known_amicables.contains(&da){
            let db = find_divisor_sum(da);
            if (a == db) && (da != a) && (a < 10000) && (da < 10000){
                known_amicables.push(a);
                known_amicables.push(da);
                amicable_sum += a + da;
            }
        }
    }
    amicable_sum
}

fn find_divisor_sum(val: i32) -> i32 {
    let mut divisor_sum = 0;
    for x in 1..=(val / 2) + 1{
        if val % x == 0 {
            divisor_sum += x;
        }
    }
    divisor_sum
}

fn main() {
    println!("{}", sum_amicable_nums(10000));
}