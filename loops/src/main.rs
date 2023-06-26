fn main() {
    let mut counter = 1;

    
    let result = 'miau: loop{
        counter *=2;
        if counter >10 {
            break 'miau counter *2;
        }
    };
    println!("The result is: {result}");
}
