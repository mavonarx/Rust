fn main() {
    let mut triangle = Vec::new();
    triangle.push(1);
    let mut divisors = Vec::new();
    divisors.push(1);

    loop {
        triangle.push(triangle[triangle.len() - 1] + triangle.len() + 1);
        if divisors.len() <= [triangle.last()] {
            divisors.push(0);
        }
        if divisors[triangle[triangle.len() - 1]] >= 500 {
            println!("{}", divisors[triangle[triangle.len() - 1]]);
            break;
        }
    }
}
