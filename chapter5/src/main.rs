use std::io;

struct Student {
    name: String,
    age: u32,
    college: String,
}

fn main() {
    let mut s1 = Student {
        name: String::from("Diego Flores"),
        age: 1,
        college: String::from("Espol"),
    };
    //println!("age {}", s1.age);
    let mut response: String = String::new();
    if user_response(){
        let mut s2 = Student{
                name: get_name(),
                age: 10,
                college: String::from("hih"),
        };
    }

    
}
fn user_response()-> bool{
    println!("Welcome, u mind answering the next questions to create ur new user Y/N");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("SOMETHING WENT WRONG");
    if response.trim()== "Y"{
        return true;
    }
    println!("OK");
    false
}

fn get_name()-> String{
    let mut name =  String::new();
    print!("ENTER YOUR NAME: -> ");
    io::stdin().read_line(&mut name).expect("SOMETHING WENT WRONG");
    name.trim().to_string()
}
fn get_age()-> Result<u32, String>{
    let mut age = String::new();
    print!("ENTER YOUR AGE: ->");
    io::stdin().read_line(&mut age).expect("SOMETHING WENT WRONG");
    let trimmed_age = age.trim().to_string();

    match trimmed_age.parse(){
        Ok(number) => Ok(number),
        Err(_) => Err(format!("THE NUMBER ENTERED IS NOT CONSIDERED VALID")),

    }
}
