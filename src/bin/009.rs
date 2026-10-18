// Problem 9: Special Pythagorean Triplet
 
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which a^2 + b^2 = c^2.
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
 
// There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.
 
// return true if the triplet is a Pythagorean triplet
 
fn is_triplet(a: i32, b: i32, c: i32) -> bool {
    if a < b && b < c {
        if a.pow(2) + b.pow(2) == c.pow(2) {
            return true
        }
    }
    false
}
 
fn find_triplets(sum_val: i32) -> (i32, i32, i32) {
    let (mut a, mut b, mut c) = (1, 2, sum_val - 3);
    while a < b {
        while b < c {
            if is_triplet(a, b, c){
                return (a, b, c)
            }
            (b, c) = (b + 1, c - 1);
        }
        (a, b) = (a + 1, a + 2);
        c = sum_val - a - b;
    }
    (a, b, c)
}
 
fn main() {
    println!("{:?}", find_triplets(1000));
}