// Problem 23: Non-Abundant Sums

// A perfect number is a number for which the sum of its proper divisors is exactly equal to 
// the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14
// = 28, which means 28 is a perfect number.

// A number n is called deficient if the sum of its proper divisors is less than n and it is 
// called abundant if this sum exceeds n.

// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can
// be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be 
// shown that all integers greater than 28123 can be written as the sum of two abundant numbers. 
// However, this upper limit cannot be reduced any further by analysis even though it is known 
// that the greatest number that cannot be expressed as the sum of two abundant numbers is less 
// than this limit.

// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

fn is_abundant(val: i32) -> bool {
    let mut div_sum: i32 = 1;
    for x in 2..=val / 2 {
        if val % x == 0 {
            div_sum += x;
        }
    }
    if div_sum > val {
        true
    }
    else {
        false
    }
}

fn find_abundant_nums(max: i32) -> Vec<i32> {
    let mut abundant_nums: Vec<i32> = Vec::new();
    for x in 1..=max {
        if is_abundant(x){
            abundant_nums.push(x);
        }
    }
    abundant_nums
}

fn find_abundant_sums(abundant_nums: Vec<i32>, max: i32) -> Vec<i32> {
    let mut abundant_sums: Vec<i32> = vec![0; max as usize];
    for x in 0..abundant_nums.len() {
        for y in x..abundant_nums.len() {
            let sum = abundant_nums[x] + abundant_nums[y];
            if sum < max {
                abundant_sums[sum as usize] = 1;
            }
        }
    }
    abundant_sums
}

fn find_non_sums_sum(abundant_sums: Vec<i32>) -> usize {
    let mut sum: usize = 0;
    for val in 1..abundant_sums.len() {
        if abundant_sums[val] == 0 {
            sum += val;
        }
    }
    sum
}

fn main() {
    let start = std::time::Instant::now();
    //println!("{:?}", find_abundant_nums(28123));
    let abundant_nums = find_abundant_nums(28123);
    let abundant_sums = find_abundant_sums(abundant_nums, 28123);
    let find_non_sums_sum = find_non_sums_sum(abundant_sums);
    println!("{}", find_non_sums_sum);
    println!("Elapsed: {:?}", start.elapsed());
}
