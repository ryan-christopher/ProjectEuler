// Problem 30: Digit Fifth Powers
// Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits: 
// 1634 = 1^4 + 6^4 + 3^4 + 4^4
// 8208 = 8^4 + 2^4 + 0^4 + 8^4 
// 9474 = 9^4 + 4^4 + 7^4 + 4^4
// As 1 = 1**4 is not a sum it is not included. The sum of these numbers is 1634 + 8208 + 9474 = 19316.
// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

fn digit_powers(pow_digit: u32) -> i32 {
    let mut stored_powers: Vec<i32> = Vec::new();
    for i in 0..=9 {
        stored_powers.push((i as i32).pow(pow_digit));
    }
    let (mut power_sum, mut val, mut max_digits) = (0, 2, 1);
    let mut max_digit_val = 9_i32.pow(pow_digit);
    while 10_i32.pow(max_digits) < max_digit_val {
        max_digits += 1;
        max_digit_val = max_digit_val + 9_i32.pow(pow_digit);
    }
    while val <= max_digit_val {
        let (mut candidate_val, mut candidate_sum) = (val.clone(), 0);
        while candidate_val > 0 {
            let current_digit: i32 = candidate_val % 10;
            candidate_sum += stored_powers[(current_digit) as usize];
            candidate_val = candidate_val / 10;
        }
        if candidate_sum == val {
            power_sum += val;
        }       
        val += 1;
    }
    power_sum
}

fn main(){
    let start = std::time::Instant::now();
    println!("{}", digit_powers(5));
    println!("Elapsed: {:?}", start.elapsed());
}