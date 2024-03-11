//ENUMS
//Option<T>

struct Professor{
    name: String,
    age: u32,

}
impl Professor{
    fn get_name(&self)->&String{
        &self.name
    }

}
struct Teacher {
    name: String,
    age: u32,

}
impl Teacher{
    fn get_name(&self)->&String{
        &self.name
    }

}
enum Education_institute{
    School(Teacher),
    College(Professor),
}

enum IpAddrKind{
    v4,
    v6,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

struct server{
    IpAdrr: IpAddrKind,
    some: u32,
}
fn print_addr(add: IpAddrKind){
    
}

fn value_in_cents(coin:&Coin)-> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 10,
        Coin::Dime =>60,
        Coin::Quarter(UsState) => {
            println!("{:?}!",UsState);
            25
        }

    }
}

fn main() {
   let four = IpAddrKind:: v4; 
    let s: Option<String> = Some(String::from("XD"));
    match s {
        Some(value)=>{
                println!("");
        }
        None => {
                println!("None");
        }
    }
    let m = Message::Write(String::from("Diego"));
    match m {
        Message::Write(s) => {
            println!("U were right {:?}!", s);
        }
        Message::Quit => {
            println!("QUIT");
        }
        Message::Move{x,y} =>{
            println!("Move");
        }
        Message::ChangeColor(color1,color2,color3)=>{
            println!("ChangeColor");
        }
    }

    let p1 = Professor{
        name: String::from("Pegaso"),
        age: 35,
    };
    let ins_t = Education_institute::College(p1);
    match ins_t{
        Education_institute::College(argument)=> {
            println!("hey my name is {}", argument.get_name());
        }
        Education_institute::School(argument) => {
            println!("hey my name is {}", argument.get_name());
        }

    }
    let ss1: Option<i32> = Some(5);

    match ss1 {
        None => println!("Is empty"),
        Some(value)=> println!("{}", value),
    }

    
}



