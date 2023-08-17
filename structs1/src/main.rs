fn main() {
    println!("Hello, world!");
    let mut user = User{
        active: true,
        username : String::from("Matthias von Arx"), // with Syntax string::from 
        email : "matthias.von-arx@gmx.ch".to_string(), //with sintay to_string()
        sing_in_count: 0,
    };
    user.active = false;
    println!("The user's name is {} and he is active? {}",
             match user.username.as_str() {
                 "Matthias von Arx" => "Ranknull",
                 _ => "l yolo heal ",
                 
             }, 
             match user.active{
                 true => "Yes",
                 false => "No",
             });

    let user2 = construct_user(true,"rank".to_string(), "my-email".to_string(), 54);
    println!("email user2 = {}", user2.email);
    let red = Color(3,2,4);
    println!("{}",red.0);
}

struct User {
    active: bool, 
    username: String,
    email: String, 
    sing_in_count: u64,
}

fn construct_user(active : bool, username : String, email: String, sing_in_count:u64)  -> User {
    User {
        active, // this is the shorthand notation instead of active: active, etc... this can be
                // done if the fields are named exactly the same as the params
        username,
        email,
        sing_in_count,
    }
}

struct Color (i32, i32, i32);


