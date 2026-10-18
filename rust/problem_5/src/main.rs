// Problem 5: Smallest Multiple

// 2520 is the smallest number that can be divided by each of the numbers from
// 1 to 10 without any remainer.

// What is the smallest positive number that is evenly divisible by all of the
// numbers from 1 to 20?

// div_by_all takes num as input and finds the smallest positive number that is
// evenly divisible by all numbers from 1 to num
fn div_by_all(num: i64) -> i64 {
    // set value equal to max number to be divided by
    let mut test_val = num;
    // begin loop to check for each value of test_val, increment by num each loop
    loop {
        for i in 2..=num {
            if test_val % i != 0 {
                break;
            }
            // if all numbers are checked, the number has been found
            else if i == num {
                return test_val;
            }
        }
        test_val += num;
    }
}

fn main() {
    println!("{:?}", div_by_all(20));
}
