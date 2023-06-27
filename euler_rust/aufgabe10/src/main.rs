// Euler aufgabe 10 
// find the sum of all primes below two million
use rayon::prelude::*;

fn main() {
    let start = 2;
    let end = 2_000_000;

    let sum: u64 = (start..=end as u64).into_par_iter().filter(|num| primechecker(*num)).sum();
    println!("The sum of all primes below two million is: {sum}");
}

fn primechecker(num: u64) -> bool {
    for i in 2..(num as f64).sqrt() as u64 +1{
        if num % i == 0{
            return false;
        } 
    }
    true
}
