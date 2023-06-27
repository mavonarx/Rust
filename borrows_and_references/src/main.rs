fn main() {
    let s1 = String::from("Butterfly");
    let len = calc_len(&s1);
    println!("The lenght of the string {0} is {1}", s1,len);

    let mut s = String::from("Hello");
    let mut_ref = &mut s;
    //let sec_mut_ref = &mut s; this is not allowd since only one mutable reference can be created
    //at the same time in the same scope
    
    //let ref2 = &s; this unmutable burrow is also not possible because there is already a mutable
    //borrow 
    
    change_string(mut_ref);
    println!("{}",s);
}

fn calc_len(s: &String)-> usize{
    s.len()
}

fn change_string(s: &mut String){
    s.push_str(", world");
}
