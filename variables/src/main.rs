fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = x + 1;
    println!("The value of x is: {x}");


    {
        let x = x * 2;
        println!("The variable x in inner scope is {x}");

    }

    println!("The variable x in outer scope is still {x}");

    let x = 2.3;
    println!("The variable x in outer scope is now float with: {x}");
    const TRHEE_HOURS_IN_SECONDS :u32 = 60*60*3;

    let remainder = -8 % 6;
    println!("remainder is {remainder}");

    let tup :(i32, u8) = (-3,2);
    let (x,y) = tup;

    let x = tup.0;

    let months = ["January", "February"];
    let array: [u32;4] = [1,2,3,4,5];
    println!("the value of the tup is: {y}");
    println!("the value of the x is {x}");
}
