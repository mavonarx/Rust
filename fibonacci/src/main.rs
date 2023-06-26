fn main() {

    let mut x = 0;
    loop {
        println!("{0} Fibonacci number is: {1}",x+1,fibonacci(x));
        if x>90 {
            break;
        }
        x +=1;
    }
}

fn fibonacci (number: i64) -> i64{
    if number == 0 {
        return 0
    }

    if number ==1 {
        return 1
    }

    let mut table :[i64; 3] = [1;3];
    table[0] = 0;

    let mut counter= 2;
    while counter <=number {
        table[2] = table[1] + table[0];
        table[0] = table[1];
        table[1] = table [2];
        counter +=1;
    }
    table [2]

}
