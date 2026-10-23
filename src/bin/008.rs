// Problem 8: Largest Product in a Series
 
// The four adjacent digits in the 1000-digit number that have the greatest product are
// 9 x 9 x 8 x 9 = 5832.
 
// Find the thirteen adjacent digits in the 1000-digit number that have the greatest
// product. What is the value of this product?
 
 
fn find_largest_product(adj_digits: u32, original_number: &str) -> (String, u128) {
    let mut product: u128 = 0;
    let mut number = String::from(original_number);
    let mut max_adjs: Vec<u32> = vec![];
    let mut candidate_adjs: Vec<u32> = vec![];
    if number.chars().count() as u32 > adj_digits {
        for _i in 0..adj_digits {
            let curr_val = number.pop();
            if curr_val.is_some(){
                candidate_adjs.push(curr_val.unwrap().to_digit(10).unwrap());
            }
        }
        max_adjs = candidate_adjs.clone();
        while number.chars().count() > 0 {
            candidate_adjs.remove(0);
            let curr_val = number.pop();
            if curr_val.is_some(){
                candidate_adjs.push(curr_val.unwrap().to_digit(10).unwrap());
            }
            let candidate_product = get_product(&candidate_adjs);
            if candidate_product > product {
                product = candidate_product;
                max_adjs = candidate_adjs.clone();
            }
        }
    }
    let mut adjs = String::new();
    for adj in max_adjs{
        adjs = adjs + &adj.to_string();
    }
    (adjs, product)
}
 
fn get_product(digits: &Vec<u32>) -> u128{
    let mut product: u128 = 1;
    for digit in digits{
        product = product * digit.clone() as u128;
    }
    product
}
 
fn main() {
    let start = std::time::Instant::now();
    let digit_1000 = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    println!("{:?}", find_largest_product(13, digit_1000));
    println!("Elapsed: {:?}", start.elapsed());
}