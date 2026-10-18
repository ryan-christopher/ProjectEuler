// Problem 24: Lexicographic Permutations

// A permutation is an ordered arrangement of objects. For example, 3124 is one possible
// permutation of the digits 1, 2, 3, and 4. If all of the permutations are listed
// numerically or alphabetically, we call it lexicographic order. The lexicographic
// permutations of 0, 1, and 2 are:

// 012, 021, 102, 120, 201, 210

// What is the millionth lexicographic permutation of the digits
// 0, 1, 2, 3, 4, 5, 6, 7, 8, and 9?

fn factorial(val: usize) -> usize {
    let mut product = 1;
    for x in 1..=val {
        product *= x;
    }
    product
}

fn find_lexicographic_permutation(mut digits: Vec::<usize>, mut p_target: usize) -> usize {
    let mut permutation = String::from("");
    p_target -= 1;
    for _x in 1..digits.len() {
        let digit = p_target / factorial(digits.len() - 1);
        permutation = format!("{}{}", &permutation, digits[digit]);
        digits.remove(digit);
        p_target %= factorial(digits.len());
    }
    if digits == [0] {
        permutation = format!("{}{}", &permutation, 0);
    }
    permutation.parse::<usize>().unwrap()
}

fn main() {
    let start = std::time::Instant::now();
    println!("{}", find_lexicographic_permutation(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 1000000));
    println!("Elapsed: {:?}", start.elapsed());
}
