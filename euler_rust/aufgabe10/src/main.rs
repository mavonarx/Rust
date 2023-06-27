// Euler aufgabe 10 
// find the sum of all primes below two million

fn main() {
    let mut sum = 0;
    for i in 1..2000000{
        if primechecker(i){
            sum +=i;
        }
    } 

    println!("The sum of all primes below two million is: {sum}");
}

fn primechecker(num: i64) -> bool {
    for i in 2..(num-1){
        if num % i == 0{
            return false;
        } 
    }
    true
}
