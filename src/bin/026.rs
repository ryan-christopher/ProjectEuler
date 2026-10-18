// Problem 26: Reciprocal Cycles

// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions
// with denominators 2 to 10 are given:
// 1 / 2 = 0.5
// 1 / 3 = 0.(3)
// 1 / 4 = 0.25
// 1 / 5 = 0.2
// 1 / 6 = 0.1(6)
// 1 / 7 = 0.(142857)
// 1 / 8 = 0.125
// 1 / 9 = 0.(1)
// 1 / 10 = 0.1

// Where 0.1(6) means 0.16666..., and has a 1-digit recurring cycle. It can be seen that 1 / 7 has a
// 6-digit recurring cycle. Find the value of d < 1000 for which 1 / d contains the longest 
// recurring cycle in its decimal fraction part.


fn longest_recurring_cycle (val: f64) {
    let mut d = 2;
    for x in 2..val as i32 {
        let fraction: f64 = 1.0 / x as f64;
        println!("x: {}, {:.}", x, fraction);
    }
}

fn main() {
    let start = std::time::Instant::now();
    longest_recurring_cycle(1000 as f64);
    println!("Elapsed: {:?}", start.elapsed());
}
